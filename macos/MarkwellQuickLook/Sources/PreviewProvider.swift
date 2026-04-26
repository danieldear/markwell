import CoreGraphics
import Foundation
import QuickLook
import UniformTypeIdentifiers

final class PreviewProvider: QLPreviewProvider {
    override func providePreview(for request: QLFilePreviewRequest) async throws -> QLPreviewReply {
        let source = try String(contentsOf: request.fileURL, encoding: .utf8)
        let html = htmlDocument(from: source, fileName: request.fileURL.lastPathComponent)
        let htmlData = Data(html.utf8)
        let size = CGSize(width: 1100, height: 1600)

        return QLPreviewReply(
            dataOfContentType: .html,
            contentSize: size
        ) { _ in
            htmlData
        }
    }
}

private func htmlDocument(from markdown: String, fileName: String) -> String {
    let escaped = escapeHTML(markdown)
    return """
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>\(escapeHTML(fileName))</title>
        <style>
          :root { color-scheme: light dark; }
          body {
            margin: 0;
            padding: 26px;
            font: 15px/1.55 -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
            background: Canvas;
            color: CanvasText;
          }
          h1 { margin: 0 0 14px; font-size: 18px; }
          pre {
            margin: 0;
            padding: 16px;
            border: 1px solid color-mix(in oklab, CanvasText 20%, Canvas 80%);
            border-radius: 8px;
            overflow: auto;
            white-space: pre-wrap;
            word-break: break-word;
            font: 13px/1.45 ui-monospace, SFMono-Regular, Menlo, monospace;
          }
        </style>
      </head>
      <body>
        <h1>\(escapeHTML(fileName))</h1>
        <pre>\(escaped)</pre>
      </body>
    </html>
    """
}

private func escapeHTML(_ input: String) -> String {
    input
        .replacingOccurrences(of: "&", with: "&amp;")
        .replacingOccurrences(of: "<", with: "&lt;")
        .replacingOccurrences(of: ">", with: "&gt;")
        .replacingOccurrences(of: "\"", with: "&quot;")
        .replacingOccurrences(of: "'", with: "&#39;")
}
