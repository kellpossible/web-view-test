use std::{time::Duration, thread};
use web_view::Content;

const HTML: &'static str = r#"
<html>
    <body>
        <h1>test</h1>
        <script type="text/javascript">
            external.invoke("exit");
        </script>
    </body>
</html>
"#;
fn main() {
    web_view::builder()
        .title("test")
        .user_data(())
        .content(Content::Html(HTML))
        .invoke_handler(|view, arg| {
            match arg {
                "exit" => {
                    println!("exit invoked from webview");
                    view.exit()
                }
                _ => {}
            }
            Ok(())
        })
        .run().unwrap();

    println!("Sleeping... Window hasn't closed!");
    thread::sleep(Duration::from_millis(5000));
}
