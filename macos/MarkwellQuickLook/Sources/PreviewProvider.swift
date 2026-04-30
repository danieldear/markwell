import CoreGraphics
import Foundation
import QuickLookUI
import UniformTypeIdentifiers

// QLPreviewProvider is the principal class for data-based Quick Look extensions.
// QLPreviewingController is the protocol that supplies the preview data.
final class PreviewProvider: QLPreviewProvider, QLPreviewingController {
    func providePreview(
        for request: QLFilePreviewRequest,
        completionHandler handler: @escaping (QLPreviewReply?, Error?) -> Void
    ) {
        do {
            let source = try String(contentsOf: request.fileURL, encoding: .utf8)
            let html = htmlDocument(from: source, fileName: request.fileURL.lastPathComponent)
            let htmlData = Data(html.utf8)
            let size = CGSize(width: 1100, height: 1600)
            let reply = QLPreviewReply(
                dataOfContentType: .html,
                contentSize: size
            ) { _ in htmlData }
            handler(reply, nil)
        } catch {
            handler(nil, error)
        }
    }
}

private func htmlDocument(from markdown: String, fileName: String) -> String {
    let body = renderMarkdown(markdown)
    return """
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>\(escapeHTML(fileName))</title>
        <style>
          :root { color-scheme: light dark; }
          *, *::before, *::after { box-sizing: border-box; }
          body {
            margin: 0;
            padding: 32px 40px;
            font: 15px/1.65 -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
            background: Canvas;
            color: CanvasText;
            max-width: 860px;
          }
          h1, h2, h3, h4, h5, h6 {
            margin: 1.4em 0 0.4em;
            font-weight: 600;
            line-height: 1.25;
          }
          h1 { font-size: 2em;   border-bottom: 1px solid color-mix(in oklab, CanvasText 15%, Canvas); padding-bottom: 0.3em; }
          h2 { font-size: 1.5em; border-bottom: 1px solid color-mix(in oklab, CanvasText 10%, Canvas); padding-bottom: 0.2em; }
          h3 { font-size: 1.25em; }
          h4 { font-size: 1em; }
          p  { margin: 0.75em 0; }
          a  { color: LinkText; text-decoration: underline; }
          code {
            font: 13px/1.4 ui-monospace, SFMono-Regular, Menlo, monospace;
            background: color-mix(in oklab, CanvasText 8%, Canvas);
            padding: 0.15em 0.35em;
            border-radius: 4px;
          }
          pre {
            margin: 1em 0;
            padding: 16px;
            border: 1px solid color-mix(in oklab, CanvasText 15%, Canvas);
            border-radius: 8px;
            overflow: auto;
            background: color-mix(in oklab, CanvasText 4%, Canvas);
          }
          pre code { background: none; padding: 0; font-size: 13px; }
          blockquote {
            margin: 1em 0;
            padding: 0.5em 1em;
            border-left: 4px solid color-mix(in oklab, CanvasText 25%, Canvas);
            color: color-mix(in oklab, CanvasText 70%, Canvas);
          }
          blockquote p { margin: 0.25em 0; }
          ul, ol { margin: 0.75em 0; padding-left: 2em; }
          li { margin: 0.3em 0; }
          hr {
            border: none;
            border-top: 1px solid color-mix(in oklab, CanvasText 15%, Canvas);
            margin: 1.5em 0;
          }
          img { max-width: 100%; height: auto; }
        </style>
      </head>
      <body>
        \(body)
      </body>
    </html>
    """
}
