use std::{time::Duration, thread};
use web_view::Content;

const HTML: &'static str = r#"
<html>
    <body>
        <h2 id="heading">Testing. This window should close after 5 seconds.</h2>
        <script type="text/javascript">
            // sleep time expects milliseconds
            function sleep (time) {
            return new Promise((resolve) => setTimeout(resolve, time));
            }

            window.onload = async function exit_after_sleep() {
                await sleep(5000);
                external.invoke("exit");
            }
        </script>
    </body>
</html>
"#;
fn main() {
    let _result = web_view::builder()
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

    println!("Sleeping... Window hasn't closed??");
    thread::sleep(Duration::from_millis(10000));
}
