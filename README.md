# xdgen
Generate localized XDG metadata for an application

## Basic usage:

Add `xdgen` as a [build-dependency](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#build-dependencies) to your project, and use the following `build.rs`:

```rust,no_run
use std::{env, fs, path::PathBuf};
use xdgen::{App, Context, FluentString};

fn main() {
    let id = "com.system76.Example";
    let ctx = Context::new(
      "i18n",
      "cosmic-example"
    ).unwrap();
    let app = App::new(FluentString("cosmic-example-name"))
        .comment(FluentString("dist-comment"))
        .keywords(FluentString("dist-keywords"));

        
    let output = PathBuf::from("target/xdgen");
    fs::create_dir_all(&output).unwrap();
    let desktop_in_file = format!("res/{}.in.desktop", id);
    println!("cargo::rerun-if-changed={desktop_in_file}");
    fs::write(
        output.join(format!("{}.desktop", id)),
        app.expand_desktop(desktop_in_file, &ctx).unwrap(),
    )
    .unwrap();

    let metainfo_in_file = format!("res/{}.metainfo.in.xml", id);
    println!("cargo::rerun-if-changed={metainfo_in_file}");
    fs::write(
        output.join(format!("{}.metainfo.xml", id)),
        app.expand_metainfo(metainfo_in_file, &ctx).unwrap(),
    )
    .unwrap();
}
```

It will look up fluent files in `i18n/[lang-id]/cosmic-example.ftl`.
From these files, `cosmic-example-name` will be used for the name of the application,
`dist-comment` will be used for the `comment` of the desktop entry and for the `summary` of the metainfo file, and `dist-keywords` will be used for the keywords.

`xdgen` needs a template for both the desktop entry and the metainfo. They must not already contain translated version of the fields you want to translate.

## Translation of all metainfo fields

In the `metainfo` template, you can add the `xdgen-translate` attribute to any element.
`xdgen` will look up the value of this attribute as a fluent identifier, and generate translations of the element.

```xml
    <release version="unreleased" date="2026-09-27">
      <description>
        <ul>
          <li xdgen-translate="release-update-dependencies">Update dependencies.</li>
          <li xdgen-translate="release-next-translate-metainfo">Translate metadata.</li>
        </ul>
      </description>
    </release>
```

Will become

```xml
    <release version="unreleased" date="2026-09-27">
      <description>
        <ul>
          <li>Update dependencies.</li>
          <li xml:lang="fr">Mise à jour des dépendances.</li>
          <li xml:lang="en">Update dependencies.</li>
          <li>Translate metadata.</li>
          <li xml:lang="fr">Traduction des métadonnées.</li>
          <li xml:lang="en">Translate metadata.</li>
        </ul>
      </description>
    </release>
```
