/*
 * generator.m -- Markwell Quick Look Generator
 *
 * Hybrid implementation: exports both the legacy C entry points
 * (GeneratePreviewForURL) AND a modern QLPreviewingController ObjC class,
 * so the bundle works on macOS 12-15 (legacy loading) and macOS 26+
 * (Extension-framework loading).
 */

#import <Foundation/Foundation.h>
#import <QuickLook/QuickLook.h>
#import <QuickLookUI/QuickLookUI.h>
#import <UniformTypeIdentifiers/UniformTypeIdentifiers.h>

#pragma clang diagnostic ignored "-Wdeprecated-declarations"

// ---------------------------------------------------------------------------
// CSS
// ---------------------------------------------------------------------------
static NSString * const kCSS =
    @":root{color-scheme:light dark}"
    "body{margin:0;padding:32px 40px;"
         "font:15px/1.65 -apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;"
         "background:Canvas;color:CanvasText;max-width:860px}"
    "h1,h2,h3,h4,h5,h6{margin:1.4em 0 .4em;font-weight:600;line-height:1.25}"
    "h1{font-size:2em;"
       "border-bottom:1px solid color-mix(in oklab,CanvasText 15%,Canvas);"
       "padding-bottom:.3em}"
    "h2{font-size:1.5em;"
       "border-bottom:1px solid color-mix(in oklab,CanvasText 10%,Canvas);"
       "padding-bottom:.2em}"
    "h3{font-size:1.25em}h4{font-size:1em}"
    "p{margin:.75em 0}"
    "a{color:LinkText;text-decoration:underline}"
    "code{font:13px/1.4 ui-monospace,SFMono-Regular,Menlo,monospace;"
         "background:color-mix(in oklab,CanvasText 8%,Canvas);"
         "padding:.15em .35em;border-radius:4px}"
    "pre{margin:1em 0;padding:16px;"
        "border:1px solid color-mix(in oklab,CanvasText 15%,Canvas);"
        "border-radius:8px;overflow:auto;"
        "background:color-mix(in oklab,CanvasText 4%,Canvas)}"
    "pre code{background:none;padding:0}"
    "blockquote{margin:1em 0;padding:.5em 1em;"
               "border-left:4px solid color-mix(in oklab,CanvasText 25%,Canvas);"
               "color:color-mix(in oklab,CanvasText 70%,Canvas)}"
    "blockquote p{margin:.25em 0}"
    "ul,ol{margin:.75em 0;padding-left:2em}"
    "li{margin:.3em 0}"
    "hr{border:none;"
       "border-top:1px solid color-mix(in oklab,CanvasText 15%,Canvas);"
       "margin:1.5em 0}"
    "img{max-width:100%;height:auto}"
    "table{border-collapse:collapse;width:100%;margin:1em 0}"
    "th,td{border:1px solid color-mix(in oklab,CanvasText 20%,Canvas);"
          "padding:6px 12px;text-align:left}"
    "th{background:color-mix(in oklab,CanvasText 6%,Canvas);font-weight:600}";

// ---------------------------------------------------------------------------
// JavaScript inline Markdown renderer
// ---------------------------------------------------------------------------
static NSString * const kJS =
    @"(function(){"
    // HTML escape
    "function e(s){return s.replace(/&/g,'&amp;').replace(/</g,'&lt;')"
                        ".replace(/>/g,'&gt;').replace(/\"/g,'&quot;')}"
    // Inline elements (two levels to avoid infinite recursion)
    "function il(s){"
    "  return s.replace(/`([^`]+)`/g,function(_,c){return'<code>'+e(c)+'</code>'})"
    "  .replace(/\\*\\*\\*(.+?)\\*\\*\\*/g,function(_,t){return'<strong><em>'+e(t)+'</em></strong>'})"
    "  .replace(/\\*\\*(.+?)\\*\\*/g,function(_,t){return'<strong>'+e(t)+'</strong>'})"
    "  .replace(/__(.+?)__/g,function(_,t){return'<strong>'+e(t)+'</strong>'})"
    "  .replace(/\\*([^\\*\\n]+?)\\*/g,function(_,t){return'<em>'+e(t)+'</em>'})"
    "  .replace(/_([^_\\n]+?)_/g,function(_,t){return'<em>'+e(t)+'</em>'})"
    "  .replace(/!\\[([^\\]]*)\\]\\(([^)]+)\\)/g,function(_,a,u){return'<img src=\"'+e(u)+'\" alt=\"'+e(a)+'\">'})"
    "  .replace(/\\[([^\\]]+)\\]\\(([^)]+)\\)/g,function(_,t,u){return'<a href=\"'+e(u)+'\">'+e(t)+'</a>'})"
    "}"
    // Block renderer
    "function render(src){"
    "  var lines=src.split('\\n'),out=[],i=0;"
    "  while(i<lines.length){"
    "    var L=lines[i];"
    "    if(!L.trim()){i++;continue;}"
    // Fenced code block
    "    var fm=L.match(/^(`{3}|~{3})(\\w*)/);"
    "    if(fm){var fence=fm[1],lang=fm[2],code=[];i++;"
    "      while(i<lines.length&&lines[i].substring(0,fence.length)!==fence)code.push(lines[i++]);"
    "      i++;"
    "      out.push('<pre><code'+(lang?' class=\"language-'+e(lang)+'\"':'')+'>'"
    "              +e(code.join('\\n'))+'</code></pre>');continue;}"
    // Heading
    "    var hm=L.match(/^(#{1,6})\\s+(.+)/);"
    "    if(hm){var lv=hm[1].length;"
    "      out.push('<h'+lv+'>'+il(hm[2])+'</h'+lv+'>');i++;continue;}"
    // HR
    "    if(/^[-*_]{3,}\\s*$/.test(L)){out.push('<hr>');i++;continue;}"
    // Blockquote
    "    if(/^>/.test(L)){var q=[];"
    "      while(i<lines.length&&/^>/.test(lines[i]))q.push(lines[i++].replace(/^>\\s?/,''));"
    "      out.push('<blockquote>'+render(q.join('\\n'))+'</blockquote>');continue;}"
    // Unordered list
    "    if(/^[-*+]\\s/.test(L)){var items=[];"
    "      while(i<lines.length&&/^[-*+]\\s/.test(lines[i]))"
    "        items.push('<li>'+il(lines[i++].replace(/^[-*+]\\s/,''))+'</li>');"
    "      out.push('<ul>'+items.join('')+'</ul>');continue;}"
    // Ordered list
    "    if(/^\\d+\\.\\s/.test(L)){var items=[];"
    "      while(i<lines.length&&/^\\d+\\.\\s/.test(lines[i]))"
    "        items.push('<li>'+il(lines[i++].replace(/^\\d+\\.\\s/,''))+'</li>');"
    "      out.push('<ol>'+items.join('')+'</ol>');continue;}"
    // Paragraph
    "    var para=[];"
    "    while(i<lines.length){"
    "      var l=lines[i];"
    "      if(!l.trim()||/^#{1,6}\\s/.test(l)||/^[-*+]\\s/.test(l)||"
    "         /^\\d+\\.\\s/.test(l)||/^>/.test(l)||/^[-*_]{3,}\\s*$/.test(l)||"
    "         /^(`{3}|~{3})/.test(l))break;"
    "      para.push(l);i++;}"
    "    if(para.length)out.push('<p>'+il(para.join(' '))+'</p>');"
    "  }"
    "  return out.join('\\n');"
    "}"
    "document.getElementById('content').innerHTML=render(window.__md);"
    "})();";

// ---------------------------------------------------------------------------
// Shared HTML builder
// ---------------------------------------------------------------------------
static NSString *buildHTML(NSString *markdown) {
    // Wrap in array so NSJSONSerialization gets a valid top-level container.
    // The result is ["<escaped markdown>"], and JS reads window.__md[0].
    NSData   *jsonData = [NSJSONSerialization dataWithJSONObject:@[markdown]
                                                         options:0
                                                           error:nil];
    NSString *jsonMD   = [[NSString alloc] initWithData:jsonData
                                               encoding:NSUTF8StringEncoding];
    return [NSString stringWithFormat:
        @"<!doctype html>"
         "<html lang=\"en\">"
         "<head>"
           "<meta charset=\"utf-8\">"
           "<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">"
           "<style>%@</style>"
         "</head>"
         "<body>"
           "<div id=\"content\"></div>"
           "<script>window.__md=%@[0];</script>"
           "<script>%@</script>"
         "</body>"
         "</html>",
        kCSS, jsonMD, kJS];
}

// ---------------------------------------------------------------------------
// Modern API: QLPreviewingController  (macOS 12+ / macOS 26 Extension system)
// ---------------------------------------------------------------------------
@interface PreviewProvider : NSObject <QLPreviewingController>
@end

@implementation PreviewProvider

- (void)providePreviewForFileRequest:(QLFilePreviewRequest *)request
                   completionHandler:(void (^)(QLPreviewReply * _Nullable,
                                               NSError * _Nullable))handler {
    NSError  *err      = nil;
    NSString *markdown = [NSString stringWithContentsOfURL:request.fileURL
                                                  encoding:NSUTF8StringEncoding
                                                     error:&err];
    if (!markdown) { handler(nil, err); return; }

    NSString *html     = buildHTML(markdown);
    NSData   *htmlData = [html dataUsingEncoding:NSUTF8StringEncoding];
    CGSize    size     = CGSizeMake(1100, 1600);

    QLPreviewReply *reply = [[QLPreviewReply alloc]
        initWithDataOfContentType:UTTypeHTML
                      contentSize:size
                 dataCreationBlock:^NSData *(QLPreviewReply *r, NSError **e) {
                     return htmlData;
                 }];
    handler(reply, nil);
}

@end

// ---------------------------------------------------------------------------
// App extension entry point — NSExtensionMain launches the XPC service and
// routes requests to NSExtensionPrincipalClass (PreviewProvider above).
// ---------------------------------------------------------------------------
extern int NSExtensionMain(int argc, char *argv[]);

int main(int argc, char *argv[]) {
    return NSExtensionMain(argc, argv);
}
