# DokuHero

Easily convert Word documents to DokuWiki

### Dochero
dochero is a CLI application that takes a `.docx` input file and an output directory.
It reads the document as a zip archive and extracts the content of the `word/document.xml`.
The `quick_xml` parser reads this XML file and returns an iterator of XML objects.
Dochero reads these XML nodes and a formatted output is pushed onto the output buffer.
Lastly the output is written to an output file.

### TODO:
- [X] Create doc_hero project
- [X] Add arg parse functionality
- [X] Add node tree building functionality
- [X] Test with simple samle XML
- [ ] Add node parse functionality
- [ ] Refactor modules
- [ ] Add web interface/API
- [ ] Create rs lib & mods
- [ ] Use FFI lib call rather than process
