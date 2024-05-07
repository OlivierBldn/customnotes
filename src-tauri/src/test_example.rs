// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;

//     #[test]
//     fn test_transform_bucket_notes() {

//         let mut map1 = HashMap::new();
//         map1.insert("timestamp".to_string(), "2024-05-05T02:51:00.732617662+00:00".to_string());
//         map1.insert("uuid".to_string(), "8d5572eb-b4a0-4697-b551-fff4de57f17e".to_string());

//         let mut map2 = HashMap::new();
//         map2.insert("timestamp".to_string(), "2024-05-05T02:19:16.798625250+00:00".to_string());
//         map2.insert("uuid".to_string(), "da1417b4-17b9-47a6-84fe-ea049d223cc3".to_string());

//         // Arrange
//         let bucket_notes = vec![
//             ("title1.txt".to_string(), Some("2024-05-05T02:51:01Z".to_string()), Some(map1), "content1".to_string()),
//             ("title2.txt".to_string(), Some("2024-05-05T02:19:17Z".to_string()), Some(map2), "content2".to_string()),
//         ];
//         let expected_output = vec![
//             (0, "title1.txt".to_string(), "content1".to_string(), String::new(), 0, None::<String>, None::<String>),
//             (0, "title2.txt".to_string(), "content2".to_string(), String::new(), 0, None::<String>, None::<String>),
//         ];

//         // Act
//         let output: Vec<_> = bucket_notes.into_iter().map(|(title, _, _, content)| {
//             (0, title, content, String::new(), 0, None, None)
//         }).collect();

//         // Assert
//         assert_eq!(output, expected_output);
//     }
// }