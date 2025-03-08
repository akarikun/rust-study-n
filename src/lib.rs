mod commons;
mod dal;
mod entities;

#[cfg(test)]
mod tests {
    use sea_orm::sqlx::types::chrono::Local;
    use std::future::IntoFuture;

    use crate::entities;

    #[test]
    fn study_get_list() -> Result<(), std::io::Error> {
        _ = crate::dal::study::get_list(1, 3);

        Ok(())
    }
    #[test]
    fn now() {
        let date = Local::now().naive_local();
        println!("{:?}", date);
    }
    #[test]
    fn study_insert_n3() {
        let level = 3;
        _ = crate::dal::study::insert(entities::study::Model {
            id: 1,
            level,
            index: 1,
            content: "日本では野球選手に<u>憧れる</u>子どもたちが多い。".to_string(),
            a: "あこがれる".to_string(),
            b: "みだれる".to_string(),
            c: "めぐまれる".to_string(),
            d: "たおれる".to_string(),
            a_label: "".to_string(),
            b_label: "".to_string(),
            c_label: "".to_string(),
            d_label: "".to_string(),
            remark: "".to_string(),
            result: 1,
            r#type: 1,
            create_date: Local::now().naive_local(),
        });
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
