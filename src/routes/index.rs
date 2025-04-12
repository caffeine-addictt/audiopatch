use axum::response::{Html, IntoResponse};

pub async fn handler() -> impl IntoResponse {
    Html(
        r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Audiopatch</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>
            body {
                background-color: #111;
                color: #eee;
                font-family: sans-serif;
                margin: 0;
                padding: 2rem;
                display: flex;
                justify-content: center;
                align-items: center;
                min-height: 100vh;
            }
            audio {
                display: block;
                margin: 1rem auto 0;
            }
            a {
                color: #66f;
                text-decoration: none;
            }
            button {
                padding: 0.5rem 1rem;
                background: #444;
                color: #eee;
                border: none;
                cursor: pointer;
                margin-bottom: 1rem;
            }
            input {
                padding: 0.5rem;
                width: 100%;
                background: #222;
                border: 1px solid #444;
                color: #eee;
                margin-bottom: 0.5rem;
                box-sizing: border-box;
            }

            .container {
                width: 100%;
                max-width: 600px;
                text-align: center;
            }

            .link-box {
                background: #222;
                padding: 0.5rem;
                border-radius: 5px;
                font-family: monospace;
                word-break: break-word;
                overflow-wrap: break-word;
                margin: 0 auto 1rem;
                max-width: 100%;
                border: 1px solid #444;
                cursor: pointer;
            }
            .link-box .base {
                color: #888;
            }
            .link-box .param-key {
                color: #0f0;
            }
            .link-box .param-eq {
                color: #555;
            }
            .link-box .param-val {
                color: #0f0;
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1 style="margin-bottom: 0.25rem;">Audiopatch</h1>
            <p style="margin-top: 0; margin-bottom: 1.5rem;">Transcode and stream audio formats for ETS2 and other tools.</p>

            <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 0.5rem; margin-bottom: 1.5rem;">
                <input id="sourceInput"
                    placeholder="Paste your audio URL (e.g. https://my-source.com/radio.m4a)"
                    style="flex: 1; min-width: 250px; padding: 0.5rem; background: #222; color: #eee; border: 1px solid #444; border-radius: 4px; height: 40px; box-sizing: border-box;">

                <select id="formatSelect"
                    style="padding: 0.5rem; background: #222; color: #eee; border: 1px solid #444; border-radius: 4px; height: 40px;">
                    <option value="mp3">mp3</option>
                </select>

                <button onclick="update()"
                    style="padding: 0 1rem; background: #333; color: #fff; border: 1px solid #444; border-radius: 4px; height: 40px; cursor: pointer;">
                    Generate
                </button>
            </div>

            <div id="resultWrapper" style="display:none; flex-direction: column; align-items: center; gap: 0.5rem; margin-bottom: 1.5rem;">
                <code onclick="copyResult()" id="result" class="link-box" title="Click to copy"></code>
                <audio id="player" controls style="display:none; width: 100%; max-width: 500px;" autoplay></audio>
            </div>

            <p style="margin-top: 2rem;"><a href="https://github.com/caffeine-addictt/audiopatch" target="_blank">View on GitHub</a></p>
            <span id="copiedMessage" style="display:none; position: absolute; right: 0; bottom: 0; margin: 0.5rem; padding: 0.25rem 0.5rem; background-color: #0f0; color: #000; border-radius: 4px; font-size: 0.9rem;">Copied!</span>
        </div>

        <script>
            let debounceTimeout;
            function copyResult() {
                const resultElement = document.getElementById("result");
                const textToCopy = resultElement.innerText;

                const tempInput = document.createElement("input");
                document.body.appendChild(tempInput);
                tempInput.value = textToCopy;
                tempInput.select();

                try {
                    document.execCommand("copy");
                    const feedback = document.getElementById("copiedMessage");
                    feedback.style.display = "inline-block";

                    clearTimeout(debounceTimeout);
                    debounceTimeout = setTimeout(() => {
                        feedback.style.display = "none";
                    }, 2000);
                } catch (err) {
                    console.error("Failed to copy text: ", err);
                }

                document.body.removeChild(tempInput);
            }

            function getStreamUrl() {
                const source = document.getElementById("sourceInput").value;
                const format = document.getElementById("formatSelect").value;
                if (!source) return;

                const url = new URL(location.href);
                url.pathname = location.pathname.replace(/\/+$/, "") + "/stream";
                url.search = `from=${encodeURIComponent(source)}&format=${encodeURIComponent(format)}`;

                return url
            }

            function updateLinkBox(url) {
                const full = url.toString();
                const [base, search] = full.split("?");
                const params = new URLSearchParams(search);

                let html = `<span class="base">${base}</span>?`;
                html += [...params.entries()]
                    .map(([k, v]) =>
                        `<span class="param-key">${k}</span><span class="param-eq">=</span><span class="param-val">${v}</span>`
                    )
                    .join('<span class="param-eq">&</span>');

                document.getElementById("result").innerHTML = html;
                document.getElementById("resultWrapper").style.display = "flex";
            }

            function update() {
                const url = getStreamUrl();
                if (!url) return;

                updateLinkBox(url);

                const player = document.getElementById("player");
                player.src = url.toString();
                player.style.display = "block";
            }
            update();
        </script>
    </body>
    </html>
    "#,
    )
}
