use ziyy::{indexer::Indexer, parser::Parser, resolver::Resolver, splitter::Splitter};

fn main() {
    let mut indexer = Indexer::new();
    let source = r#"<ziyy>
        <let name="bold:green" c="rgb(0,150,75)" b u />
        <let name="cyan" c="rgb(0,150,150)" />

        <p>{}</p>
        <br />
        <p>
            <u src="bold:green">Usage:</u> <cyan><b>ziyy</b> <i>[OPTION]</i> \<FILE\></cyan>
        </p>
        <br />

        <p src="bold:green">Options:</p>
        <p tab="2" src="cyan" b>-V<e>,</e> --version</p>
        <p tab="10">Print version info and exit</p>
        <p tab="2" src="cyan" b>-h<e>,</e> --help</p>
        <p tab="10">Print help</p>
        <br /> [1;3;48;2;150;75;0mHello World! \x1b[m
    </ziyy>"#;
    let source = indexer.index(source.to_string());
    println!("{:?}", source);

    let mut splitter = Splitter::new();
    let frags = splitter.split(source);
    println!("{:#?}", frags);

    let parser = Parser::new();
    let chunks = parser.parse(frags).unwrap();
    println!("{:#?}", chunks);

    let mut resolver = Resolver::new();
    let output = resolver.resolve(chunks);

    eprint!("{output}");

    let mut buf = String::new();
    output.root().to_string(&mut buf);
    eprint!("{buf}");
}
