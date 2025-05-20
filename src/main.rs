use ziyy::{stage_1, stage_2, stage_3};

fn main() {
    let mut stage_1 = stage_1::Stage1::new();
    let source = r#"<ziyy>
        <let name="bold:green" c="rgb({},{},{})" b u />
        <let name="cyan" c="rgb({r},{g},{g})" />

        <p>{}</p>
        <br />
        <p>
            <u src="bold:green">Usage:</u> <cyan><b>ziyy</b> <i>[OPTION]</i> \<FILE\></cyan> \\
        </p>
        <br />

        <p src="bold:green">Options:</p>
        <p tab="2" src="cyan" b>-V<e>,</e> --version</p>
        <p tab="10">Print version info and exit</p>
        <p tab="2" src="cyan" b>-h<e>,</e> --help</p>
        <p tab="10">Print help</p>
        <br /> [1;3;48;2;150;75;0mHello World!
    </ziyy>"#;
    let parts = stage_1.parse(source.to_string());
    println!("{:?}", parts);

    let mut stage_2 = stage_2::Stage2::new();
    let frags = stage_2.parse(parts);
    println!("{:#?}", frags);

    let stage_3 = stage_3::Stage3::new();
    let chunks = stage_3.parse(frags).unwrap();
    println!("{:#?}", chunks);

    for chunk in chunks {
        eprint!("{chunk}")
    }
}
