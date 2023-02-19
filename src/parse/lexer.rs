use crate::{logic::{KeyValue, LogicChain, Logical}, recipe::{SideEffect, AnyRecipe}, key::Key, Recipe};
use nom_prelude::{complete::*, *};

pub(crate) fn any_recipe<'a, E: ParseError<&'a str>>(ref mut i: &'a str) -> IResult<&'a str, AnyRecipe<&'a str>, E> {
    alt((
        map(preceded(char('!'), cut(numeric_recipe)), AnyRecipe::Numeric),
        map(recipe, AnyRecipe::Textual),
    ))(i)
}

fn recipe<'a, E: ParseError<&'a str>>(ref mut i: &'a str) -> IResult<&'a str, Recipe<&'a str, &'a str>, E> {
    let entry = Recipe {
        name: apply(i, terminated(not_a_dog, a_dog))?,
        description: apply(i, terminated(opt(not_a_dog), a_dog))?,
        params_to_see: apply(i, optional_dog_logic_chain)?,
        params_to_craft: apply(i, optional_dog_logic_chain)?,
        ingredients: apply(i, dog_logic_chain)?,
        tools: apply(i, optional_dog_logic_chain)?,
        output: apply(i, dog_logic_chain)?,
        side_effect: apply(i, side_effect)?,
    };
    Ok((i, entry))
}

fn numeric_recipe<'a, E: ParseError<&'a str>>(ref mut i: &'a str) -> IResult<&'a str, Recipe<&'a str, u32>, E> {
    let entry = Recipe {
        name: apply(i, terminated(not_a_dog, a_dog))?,
        description: apply(i, terminated(opt(not_a_dog), a_dog))?,
        params_to_see: apply(i, optional_numeric_logic_chain)?,
        params_to_craft: apply(i, optional_numeric_logic_chain)?,
        ingredients: apply(i, numeric_logic_chain::<true, _>)?,
        tools: apply(i, optional_numeric_logic_chain)?,
        output: apply(i, numeric_logic_chain::<false, _>)?,
        side_effect: apply(i, side_effect)?,
    };
    Ok((i, entry))
}

fn logic_chain<'a, E: ParseError<&'a str>, K: Key<'a>>(
    ref mut i: &'a str,
) -> IResult<&'a str, LogicChain<K>, E> {
    let chain = LogicChain {
        first: apply(i, key_value)?,
        rest: apply(i, many0(pair(logical, key_value)))?,
    };
    Ok((i, chain))
}

fn key_value<'a, 'b, E: ParseError<&'a str>, K: Key<'a>>(i: &'a str) -> IResult<&'a str, KeyValue<K>, E> {
    map_res(
        space0_delimited(separated_pair(word, space1, unsigned_number)),
        |(key, value)| {
            K::key_from(key).ok_or(()).map(|key| KeyValue { key, value })
        },
    )(i)
}

fn logical<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Logical, E> {
    space0_delimited(alt((
        map(char('&'), |_| Logical::And),
        map(char('|'), |_| Logical::Or),
    )))(i)
}

fn side_effect<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, SideEffect<&'a str>, E> {
    alt((
        map(
            preceded(
                pair(tag("script"), space1),
                separated_pair(not_a_dog, a_dog, word),
            ),
            |(module, function)| SideEffect::Script { module, function },
        ),
        map(preceded(pair(tag("exp"), space1), unsigned_number), |exp| {
            SideEffect::Experience(exp)
        }),
        map(terminated(tag("script"), eof), |_| SideEffect::Script{module: "", function: ""}),
        map(terminated(tag("exp"), eof), |_| SideEffect::Experience(0)),
    ))(i)
}

fn not_a_dog<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_till1(|ch| ch == '@')(i)
}

fn a_dog<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, char, E> {
    char('@')(i)
}

fn dog_logic_chain<'a, E: ParseError<&'a str>, K: Key<'a>>(i: &'a str) -> IResult<&'a str, LogicChain<K>, E> {
    terminated(map_parser(not_a_dog, logic_chain), a_dog)(i)
}

fn optional_dog_logic_chain<'a, E: ParseError<&'a str>, K: Key<'a>>(
    i: &'a str,
) -> IResult<&'a str, Option<LogicChain<K>>, E> {
    terminated(opt(map_parser(not_a_dog, logic_chain)), a_dog)(i)
}

fn numeric_logic_chain<'a, const ORS: bool, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, LogicChain<u32>, E> {
    let (i, res) = optional_numeric_logic_chain_ext::<ORS, _>(i)?;
    match res {
        Some(ok) => Ok((i, ok)),
        None => Err(nom::Err::Failure(ParseError::from_error_kind(i, ErrorKind::NonEmpty)))
    }
}

fn optional_numeric_logic_chain<'a, E: ParseError<&'a str>>(
    ref mut i: &'a str,
) -> IResult<&'a str, Option<LogicChain<u32>>, E> {
    optional_numeric_logic_chain_ext::<true, _>(i)
}

fn optional_numeric_logic_chain_ext<'a, const ORS: bool, E: ParseError<&'a str>>(
    ref mut i: &'a str,
) -> IResult<&'a str, Option<LogicChain<u32>>, E> {
    let nums = apply(i, spacenums)?;
    let vals = apply(i, spacenums)?;
    let ors = if ORS {
        apply(i, spacenums)?
    } else {
        vec![0; nums.len()]
    };
    let mut kv = nums.into_iter().zip(vals.into_iter());
    let Some(first) = kv.next() else {
        return Ok((i, None));
    };
    let iter = kv.zip(ors.into_iter()).map(|((key, value), or)| {
        (if or == 0 {
            Logical::And
        } else {
            Logical::Or
        }, KeyValue{key, value})
    });
    Ok((i, Some(LogicChain {
        first: KeyValue{key: first.0, value: first.1}, rest: iter.collect(),
    })))
}

fn spacenums<'a, E: ParseError<&'a str>>(
    ref mut i: &'a str,
) -> IResult<&'a str, Vec<u32>, E> {
    let len: u32 = apply(i, terminated(unsigned_number, space0))?;
    cut(many_m_n(len as usize, len as usize, terminated(unsigned_number, space0)))(i)
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::{book::RecipeBook};

    use super::*;

    fn lex<'a, T: 'a, F>(fun: F, str: &'a str) -> T
    where
        F: FnOnce(&'a str) -> IResult<&'a str, T, nom::error::VerboseError<&'a str>>,
    {
        match fun(str) {
            Ok(ok) => Ok(ok.1),
            Err(err) => Err(err.map(|err| {
                println!("{}", nom::error::convert_error(str, err.clone()));
                err
            })),
        }.unwrap()
    }

    #[test]
    fn test_side_effect_script() {
        const SAMPLE: &str = "script fix_boy@fix_Tribal";
        let correct = SideEffect::Script {
            module: "fix_boy",
            function: "fix_Tribal",
        };
        assert_eq!(correct, lex(side_effect, SAMPLE));
    }

    #[test]
    fn test_side_effect_exp() {
        const SAMPLE: &str = "exp 100";
        let correct = SideEffect::Experience(100);
        assert_eq!(correct, lex(side_effect, SAMPLE));
    }

    #[test]
    fn test_zaplatka() {
        const SAMPLE: &str = "\
            PID_ZAPLATKA_CRAFT_BASIC@Разделитель раздела для создания простейших вещей.\
            @@@PID_ZAPLATKA_CRAFT_BASIC 1@@PID_ZAPLATKA_CRAFT_BASIC 1@script fix_boy@fix_Tribal\
        ";
        let correct = Recipe {
            name: "PID_ZAPLATKA_CRAFT_BASIC",
            description: Some("Разделитель раздела для создания простейших вещей."),
            params_to_see: None,
            params_to_craft: None,
            ingredients: LogicChain {
                first: KeyValue {
                    key: "PID_ZAPLATKA_CRAFT_BASIC",
                    value: 1,
                },
                rest: vec![],
            },
            tools: None,
            output: LogicChain {
                first: KeyValue {
                    key: "PID_ZAPLATKA_CRAFT_BASIC",
                    value: 1,
                },
                rest: vec![],
            },
            side_effect: SideEffect::Script {
                module: "fix_boy",
                function: "fix_Tribal",
            },
        };
        assert_eq!(correct, lex(recipe, SAMPLE));
    }

    #[test]
    fn test_jet() {
        const SAMPLE: &str = "\
            PID_EMPTY_JET@Пустая банка для Джета. Расплавьте пластиковую бутылку и полученную жидкость влейте в форму, \
            после чего из различного мусора соберите простейший клапан и закрепите на еще горячем пластике.\
            @@SK_REPAIR 100|SK_SCIENCE 100@PID_BOTTLE_EMPTY 5&PID_CRAFT_L_LINT 5&PID_CRAFT_M_JUNK 1@PID_KNIFE 1&PID_LIGHTER 1@PID_EMPTY_JET 5\
            @script fix_boy@fix_FreeHands
        ";
        let correct = Recipe {
            name: "PID_EMPTY_JET",
            description: Some("Пустая банка для Джета. Расплавьте пластиковую бутылку и полученную жидкость влейте в форму, \
            после чего из различного мусора соберите простейший клапан и закрепите на еще горячем пластике."),
            params_to_see: None,
            params_to_craft: Some(LogicChain {
                first: KeyValue {
                    key: "SK_REPAIR",
                    value: 100,
                },
                rest: vec![
                    (Logical::Or, KeyValue {
                        key: "SK_SCIENCE",
                        value: 100,
                    }),
                ],
            }),
            ingredients: LogicChain {
                first: KeyValue {
                    key: "PID_BOTTLE_EMPTY",
                    value: 5,
                },
                rest: vec![
                    (Logical::And, KeyValue {
                        key: "PID_CRAFT_L_LINT",
                        value: 5,
                    }),
                    (Logical::And, KeyValue {
                        key: "PID_CRAFT_M_JUNK",
                        value: 1,
                    }),
                ],
            },
            tools: Some(LogicChain {
                first: KeyValue {
                    key: "PID_KNIFE",
                    value: 1,
                },
                rest: vec![
                    (Logical::And, KeyValue {
                        key: "PID_LIGHTER",
                        value: 1,
                    }),
                ],
            }),
            output: LogicChain {
                first: KeyValue {
                    key: "PID_EMPTY_JET",
                    value: 5,
                },
                rest: vec![],
            },
            side_effect: SideEffect::Script {
                module: "fix_boy",
                function: "fix_FreeHands",
            },
        };
        assert_eq!(correct, lex(recipe, SAMPLE));
    }

    #[cfg(feature = "display")]
    fn assert_eq_display(original: &str, new: &str) {
        use crate::{logic::LogicNode, display::{LogicFmtConfig, LogicDisplay}};
        const CONFIG: LogicFmtConfig = LogicFmtConfig::basic();
        let logic_chain: LogicChain<&str> = lex(logic_chain, original);
        assert_eq!(new, logic_chain.display(&CONFIG));
        let logic_nodes: LogicNode<&str> = logic_chain.logic_nodes();
        assert_eq!(new, logic_nodes.display(&CONFIG));
    }

    #[test]
    #[cfg(feature = "display")]
    fn test_logic_chain_display() {
        assert_eq_display("SK_REPAIR 100", "SK_REPAIR: 100");
        assert_eq_display(
            "SK_REPAIR 100|SK_SCIENCE 100",
            "SK_REPAIR: 100 or SK_SCIENCE: 100",
        );
        assert_eq_display(
            "SK_REPAIR 100|SK_SCIENCE 100&SK_DOCTOR 100",
            "(SK_REPAIR: 100 or SK_SCIENCE: 100) and SK_DOCTOR: 100",
        );
        assert_eq_display(
            "SK_REPAIR 100&SK_SCIENCE 100|SK_DOCTOR 100",
            "SK_REPAIR: 100 and (SK_SCIENCE: 100 or SK_DOCTOR: 100)",
        );
        assert_eq_display(
            "SK_REPAIR 100|SK_SCIENCE 100&SK_DOCTOR 100|SK_OUTDOORSMAN 100",
            "(SK_REPAIR: 100 or SK_SCIENCE: 100) and (SK_DOCTOR: 100 or SK_OUTDOORSMAN: 100)",
        );
    }

    #[test]
    fn lex_forp_crafts() {
        for dir in &["../../../FO4RP/text/engl"] {
            for file in std::fs::read_dir(dir).unwrap() {
                let path = file.unwrap().path();
                if path.file_name().unwrap() == "FOCRAFT.MSG" {
                    let path_str = path.to_str().unwrap();
                    let res = fo_msg_format::parse_cp1251_file(&path).expect(path_str);
                    let _book = RecipeBook::<Recipe<&str, &str>>::try_from_iter(res.iter_firsts()).unwrap();
                    let book = RecipeBook::<crate::NodeRecipe<String, String>>::try_from_iter(res.iter_firsts()).unwrap();
                    dbg!(book);
                }
            }
        }
    }

    #[test]
    fn net_meat_jerky() {
        let net = "!PID_MEAT_JERKY@Meat dried over a fire pit.@0 0 1 0 1 217 1 100 1 0 2 1440 125 2 4 1 2 0 0 1 3979 1 1 1 0 2 284 542 2 3 1 script";
        let res_net = lex(any_recipe, net);
        dbg!(res_net);
    }

    fn meat_jerkies() -> (Recipe<&'static str, u32>, Recipe<&'static str, &'static str>) {
        let net = "!PID_MEAT_JERKY@Meat dried over a fire pit.@0 0 1 0 1 217 1 100 1 0 2 1440 125 2 4 1 2 0 0 1 3979 1 1 1 0 2 284 542 2 3 1 script";
        let file = "PID_MEAT_JERKY@Meat dried over a fire pit.@@SK_OUTDOORSMAN 100@PID_RAD_MEAT 4&PID_SPIRIT 1@PID_FIREPLACE_TOKEN 1@PID_MEAT_JERKY 3&PID_BOTTLE_GLASS 1@script fix_boy@fix_Tribal";

        let res_net = lex(any_recipe, net);
        let res_file = lex(any_recipe, file);
        
        let numeric: Recipe<&str, u32> = res_net.try_into().unwrap();
        let textual: Recipe<&str, &str> = res_file.try_into().unwrap();
        (numeric, textual)
    }

    #[test]
    fn sameish_meat_jerky() {
        let (numeric, textual) = meat_jerkies();

        assert_eq!(numeric.name, textual.name);
        assert_eq!(numeric.description, textual.description);
        fn assert_eq_chain<K, K2>(a: Option<LogicChain<K>>, b: Option<LogicChain<K2>>) {
            let (Some(a), Some(b)) = (&a, &b) else {
                assert_eq!(a.is_none(), b.is_none());
                return
            };
            assert_eq!(a.first.value, b.first.value);
            assert_eq!(a.rest.len(), b.rest.len());
            for ((a_logical, a_kv),(b_logical, b_kv)) in a.rest.iter().zip(b.rest.iter()) {
                assert_eq!(a_logical, b_logical);
                assert_eq!(a_kv.value, b_kv.value);
            }
        }
        assert_eq_chain(numeric.params_to_see, textual.params_to_see);
        assert_eq_chain(numeric.params_to_craft, textual.params_to_craft);
        assert_eq_chain(numeric.tools, textual.tools);
        assert_eq_chain(Some(numeric.ingredients), Some(textual.ingredients));
        assert_eq_chain(Some(numeric.output), Some(textual.output));
    }

    #[test]
    fn same_converted_meat_jerky() {
        let lst = fo_lst_format::parse_dir("../../../FO4RP/data").unwrap();
        let (numeric, textual) = meat_jerkies();

        let mut converted = textual.with_keys(|key, _meaning| lst.string_to_index(*key).ok_or_else(|| format!("Key {key} not found in dictionary"))).unwrap();
        converted.side_effect = converted.side_effect.truncated();

        assert_eq!(numeric, converted);
    }

    #[test]
    fn same_reverse_converted_meat_jerky() {
        let lst = fo_lst_format::parse_dir("../../../FO4RP/data").unwrap();
        let (numeric, mut textual) = meat_jerkies();
        textual.side_effect = textual.side_effect.truncated();

        let converted = numeric.with_keys(|key, meaning| {
            lst.index_to_string_in_file(*key, meaning.lst_file_name()).ok_or_else(|| format!("Key {key} not found in dictionary"))
        }).unwrap();

        assert_eq!(textual, converted);
    }

    #[test]
    fn convert_to_node_recipe() {
        let (numeric, textual) = meat_jerkies();
        let _textual: crate::NodeRecipe<&str, &str> = textual.into();
        let _numeric: crate::NodeRecipe<&str, u32> = numeric.into();
    }
}
