use header42_lsp::comments::get_delimiters;
use header42_lsp::header::{detect_header, generate_header, generate_line, update_header_timestamp};

#[test]
fn test_c_header_dimensions_and_structure() {
    let delim = get_delimiters("main.c");
    let header = generate_header(
        "main.c",
        "login",
        "login@student.42.fr",
        "2026/08/18 20:10:00",
        "2026/08/18 20:10:00",
        &delim,
    );

    let lines: Vec<&str> = header.lines().collect();
    assert_eq!(lines.len(), 11, "Header must be exactly 11 lines");

    for (i, line) in lines.iter().enumerate() {
        assert_eq!(
            line.len(),
            80,
            "Line {} length was {}, expected 80. Content: '{}'",
            i + 1,
            line.len(),
            line
        );
    }

    assert_eq!(lines[0], "/* ************************************************************************** */");
    assert_eq!(lines[1], "/*                                                                            */");
    assert_eq!(lines[2], "/*                                                        :::      ::::::::   */");
    assert_eq!(lines[3], "/*   main.c                                             :+:      :+:    :+:   */");
    assert_eq!(lines[4], "/*                                                    +:+ +:+         +:+     */");
    assert_eq!(lines[5], "/*   By: login <login@student.42.fr>                +#+  +:+       +#+        */");
    assert_eq!(lines[6], "/*                                                +#+#+#+#+#+   +#+           */");
    assert_eq!(lines[7], "/*   Created: 2026/08/18 20:10:00 by login             #+#    #+#             */");
    assert_eq!(lines[8], "/*   Updated: 2026/08/18 20:10:00 by login            ###   ########.fr       */");
    assert_eq!(lines[9], "/*                                                                            */");
    assert_eq!(lines[10], "/* ************************************************************************** */");
}

#[test]
fn test_python_header_dimensions() {
    let delim = get_delimiters("script.py");
    let header = generate_header(
        "script.py",
        "marvin",
        "marvin@42.fr",
        "2026/08/18 20:10:00",
        "2026/08/18 20:10:00",
        &delim,
    );

    let lines: Vec<&str> = header.lines().collect();
    assert_eq!(lines.len(), 11);

    for (i, line) in lines.iter().enumerate() {
        assert_eq!(
            line.len(),
            79,
            "Python Line {} length was {}, expected 79. Content: '{}'",
            i + 1,
            line.len(),
            line
        );
    }

    assert_eq!(lines[0], "# *************************************************************************** #");
    assert_eq!(lines[10], "# *************************************************************************** #");
}

#[test]
fn test_header_detection_and_update() {
    let delim = get_delimiters("main.c");
    let original_header = generate_header(
        "main.c",
        "login",
        "login@student.42.fr",
        "2026/08/18 20:10:00",
        "2026/08/18 20:10:00",
        &delim,
    );
    let full_content = format!("{}\n\nint main() {{ return 0; }}\n", original_header);

    let detected = detect_header(&full_content, &delim);
    assert!(detected.is_some(), "Header should be detected");
    let info = detected.unwrap();
    assert_eq!(info.filename, "main.c");
    assert_eq!(info.created_date, "2026/08/18 20:10:00");
    assert_eq!(info.created_user, "login");

    let updated_content = update_header_timestamp(&full_content, "newuser", &delim);
    assert!(updated_content.is_some());
    let updated = updated_content.unwrap();
    let updated_lines: Vec<&str> = updated.lines().collect();
    assert_eq!(updated_lines.len(), 13);
    assert!(updated_lines[8].contains("Updated: "));
    assert!(updated_lines[8].contains("by newuser"));
    // Ensure created line was not altered
    assert_eq!(updated_lines[7], "/*   Created: 2026/08/18 20:10:00 by login             #+#    #+#             */");
}

#[test]
fn test_generate_single_line_9() {
    let delim = get_delimiters("main.c");
    let line9 = generate_line(9, "main.c", "login", "login@student.42.fr", "2026/08/18 20:10:00", &delim);
    assert_eq!(line9.len(), 80);
    assert_eq!(line9, "/*   Updated: 2026/08/18 20:10:00 by login            ###   ########.fr       */");
}
