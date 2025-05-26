#import "@preview/touying:0.6.1": *
#import themes.university: *
#import "@preview/cetz:0.3.4"
#import "@preview/fletcher:0.5.5" as fletcher: node, edge
#import "@preview/numbly:0.1.0": numbly
#import "@preview/theorion:0.3.2": *
#import cosmos.clouds: *
#show: show-theorion

#let cetz-canvas = touying-reducer.with(reduce: cetz.canvas, cover: cetz.draw.hide.with(bounds: true))
#let fletcher-diagram = touying-reducer.with(reduce: fletcher.diagram, cover: fletcher.hide)

#let touying-theme-nju(
  ..args,
  body,
) = {
  show: university-theme.with(
    aspect-ratio: "16-9",

    header-right: self => {
      (image("NJU.svg", height: 1.5em))
    },

    // align: horizon,
    // config-common(handout: true),

    config-common(
      frozen-counters: (theorem-counter,),
      new-section-slide-fn: new-section-slide.with(
        config: config-page(background: place(center + horizon, dx: 50%, dy: 0%, image("NJU-bg.svg", width: 50%))),
      ),
    ), // freeze theorem counter for animation

    config-colors(
      primary: rgb("#63065f"),
      secondary: rgb("#912c8c"),
      tertiary: rgb("#c646bd"),
      neutral-lightest: rgb("#ffffff"),
      neutral-darkest: rgb("#000000"),
    ),
    ..args,
  )
  let font = (
    main: "Libertinus Serif",
    mono: "Fira Code",
    cjk: "Noto Serif CJK SC",
  )

  set text(font: (font.main, font.cjk), ligatures: true)

  show raw: set text(font: (font.mono, font.cjk), ligatures: true)

  set heading(numbering: numbly("{1}.", default: "1.1"))
  body
}
