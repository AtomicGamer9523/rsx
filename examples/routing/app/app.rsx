use rsx::*;

#[page]
rsx! {
    <html>
        <head>
            <title>"Hello World"</title>
            <meta name=types::Metadata::Author content="Developer Co."/>
        </head>
        <body>
            <h1>"Hi"</h1>
            <p class="official">
                "The tool company for tool companies"
            </p>
            {
                (0..4).map(|_|
                    rsx!(
                        <p class="emphasis">
                            ">o_o<"
                        </p>
                    )
                )
            }
            <p class="citation-needed">
                "Every company should be a developer experience company"
            </p>
        </body>
    </html>
}