use askama::Template;

#[derive(Template)]
#[template(source = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CheshireLane</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gradient-to-br from-blue-800 to-blue-600 flex items-center justify-center h-screen">
    <div class="text-center text-white">
        <h1 class="text-5xl font-bold mb-4">CheshireLane</h1>
        <p class="text-2xl font-light">
            A proof-of-concept PS for game Azur Lane made by
            <a href="https://github.com/Irminsul-dev" class="underline">Irminsul.dev</a>
        </p>
    </div>
</body>
</html>
"#, ext = "html")]
pub struct IndexPage;
