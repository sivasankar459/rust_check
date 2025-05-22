use std::result;

use TextEditor::TextEditor;

#[test]
fn test_add_text_and_get_content(){
    let mut file =  TextEditor::new();
    file.add_text("Hello my name is sivasankar, How are you?");
    assert_eq!(file.get_content(),"Hello my name is sivasankar, How are you?");
}

#[test]
fn test_remove_text_replace_text(){
    let mut file =  TextEditor::new();
    file.add_text("Hello my name is sivasankar, How are you?");
    file.remove_text("sivasankar");
    assert_eq!(file.get_content(),"Hello my name is , How are you?");
    file.replace_text("my name is ,","my name is karthick,");
}

#[test]
fn test_read_file_save_file_get_line(){
    let result = TextEditor::open_file("sample.txt");
    assert!(result.is_ok());

    let mut editor = result.unwrap();
    editor.add_text("\nnew line added.");

    editor.save_to_file("new_file.txt");

    let result = editor.read_file("sample.txt");
    println!("{:?}",result.unwrap().get_content());

    let line =TextEditor::get_line("sample.txt",5);
    assert_eq!(line.unwrap().as_str(),"rust developer");

}