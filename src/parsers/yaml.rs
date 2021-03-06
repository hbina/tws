#[derive(Debug)]
pub struct YamlSolver;

impl YamlSolver {
    fn resolve_and_replace_value<'a>(
        value: &'a mut serde_yaml::Value,
        expression: &Vec<&str>,
        replace: serde_yaml::Value,
    ) -> Result<(), crate::TError> {
        let mut reader = &mut *value;
        for expr in expression.iter() {
            if let Ok(index) = expr.parse::<usize>() {
                reader = reader
                    .get_mut(index)
                    .ok_or_else(|| crate::TError::KeyNotExist(expr.to_string()))?;
                reader = reader
                    .get_mut(expr)
                    .ok_or_else(|| crate::TError::KeyNotExist(expr.to_string()))?;
            }
        }
        *reader = replace;
        Ok(())
    }

    fn resolve_value<'a>(
        value: &'a serde_yaml::Value,
        expression: &Vec<&str>,
    ) -> Result<&'a serde_yaml::Value, crate::TError> {
        let mut reader = value;
        for expr in expression.iter() {
            if let Ok(index) = expr.parse::<usize>() {
                reader = reader
                    .get(index)
                    .ok_or_else(|| crate::TError::KeyNotExist(expr.to_string()))?;
            } else {
                reader = reader
                    .get(expr)
                    .ok_or_else(|| crate::TError::KeyNotExist(expr.to_string()))?;
            }
        }
        Ok(reader)
    }

    fn value_to_string(value: &serde_yaml::Value) -> String {
        match value {
            serde_yaml::Value::String(s) => s.clone(),
            o => serde_yaml::to_string(o).unwrap(),
        }
    }
}

impl crate::Solver for YamlSolver {
    fn solve(input: &str, expression: Option<&str>, replace: Option<&str>) -> String {
        let mut json_value = serde_yaml::from_str::<serde_yaml::Value>(&input)
            .map_err(|x| crate::TError::ConversionError(input.to_string(), Box::new(x)))
            .unwrap();
        let expression = if let Some(expression) = expression {
            expression.split('.').collect()
        } else {
            vec![]
        };
        if let Some(replace) = replace {
            let replace_value = serde_yaml::from_str::<serde_yaml::Value>(&replace)
                .map_err(|x| crate::TError::ConversionError(input.to_string(), Box::new(x)))
                .unwrap();
            YamlSolver::resolve_and_replace_value(&mut json_value, &expression, replace_value)
                .unwrap();
            YamlSolver::value_to_string(&json_value)
        } else {
            let resolved_value = YamlSolver::resolve_value(&json_value, &expression).unwrap();
            YamlSolver::value_to_string(resolved_value)
        }
    }
}
