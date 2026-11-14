use xdgen::{App, Context, FluentString};

#[test]
fn testing() {
    let id = "com.system76.Example";
    let ctx = Context::new("tests/i18n", "cosmic-example").unwrap();
    let app = App::new(FluentString("cosmic-example"))
        .comment(FluentString("comment"))
        .keywords(FluentString("keywords"));
    let desktop_generated = app
        .expand_desktop(format!("tests/{}.in.desktop", id), &ctx)
        .unwrap();
    println!("{desktop_generated}");
    let desktop_expected = include_str!("com.system76.Example.out.desktop");
    assert_eq!(desktop_expected, desktop_generated);

    let metainfo_generated = app
        .expand_metainfo(format!("tests/{}.metainfo.in.xml", id), &ctx)
        .unwrap();
    println!("{metainfo_generated}");
    let metainfo_expected = include_str!("com.system76.Example.metainfo.out.xml");
    assert_eq!(metainfo_expected, metainfo_generated,);
}
