use plotly::{
    common::{
        AxisSide,
        //DashType,
        Font,
        HoverInfo,
        Marker,
        MarkerSymbol,
        Mode,
        Side,
        Title,
    },
    layout::Axis,
    Layout, Plot, Scatter,
};

use rand::Rng;
use serde::Serialize;

mod context;
pub use context::PlotContext;

use cggtts::prelude::Epoch;

/*
 * Generates N marker symbols to be used
 * to differentiate data
 */
pub fn generate_markers(n: usize) -> Vec<MarkerSymbol> {
    //TODO lazy static
    let pool = vec![
        "Circle",
        "CircleOpen",
        "CircleDot",
        "CircleOpenDot",
        "Square",
        "SquareOpen",
        "SquareDot",
        "SquareOpenDot",
        "Diamond",
        "DiamondOpen",
        "DiamondDot",
        "DiamondOpenDot",
        "Cross",
        "CrossOpen",
        "CrossDot",
        "CrossOpenDot",
        "X",
        "XOpen",
        "XDot",
        "XOpenDot",
        "TriangleUp",
        "TriangleUpOpen",
        "TriangleUpDot",
        "TriangleUpOpenDot",
        "TriangleDown",
        "TriangleDownOpen",
        "TriangleDownDot",
        "TriangleDownOpenDot",
        "TriangleLeft",
        "TriangleLeftOpen",
        "TriangleLeftDot",
        "TriangleLeftOpenDot",
        "TriangleRight",
        "TriangleRightOpen",
        "TriangleRightDot",
        "TriangleRightOpenDot",
        "TriangleNE",
        "TriangleNEOpen",
        "TriangleNEDot",
        "TriangleNEOpenDot",
        "TriangleSE",
        "TriangleSEOpen",
        "TriangleSEDot",
        "TriangleSEOpenDot",
        "TriangleSW",
        "TriangleSWOpen",
        "TriangleSWDot",
        "TriangleSWOpenDot",
        "TriangleNW",
        "TriangleNWOpen",
        "TriangleNWDot",
        "TriangleNWOpenDot",
        "Pentagon",
        "PentagonOpen",
        "PentagonDot",
        "PentagonOpenDot",
        "Hexagon",
        "HexagonOpen",
        "HexagonDot",
        "HexagonOpenDot",
        "Hexagon2",
        "Hexagon2Open",
        "Hexagon2Dot",
        "Hexagon2OpenDot",
        "Octagon",
        "OctagonOpen",
        "OctagonDot",
        "OctagonOpenDot",
        "Star",
        "StarOpen",
        "StarDot",
        "StarOpenDot",
        "Hexagram",
        "HexagramOpen",
        "HexagramDot",
        "HexagramOpenDot",
        "StarTriangleUp",
        "StarTriangleUpOpen",
        "StarTriangleUpDot",
        "StarTriangleUpOpenDot",
        "StarTriangleDown",
        "StarTriangleDownOpen",
        "StarTriangleDownDot",
        "StarTriangleDownOpenDot",
        "StarSquare",
        "StarSquareOpen",
        "StarSquareDot",
        "StarSquareOpenDot",
        "StarDiamond",
        "StarDiamondOpen",
        "StarDiamondDot",
        "StarDiamondOpenDot",
        "DiamondTall",
        "DiamondTallOpen",
        "DiamondTallDot",
        "DiamondTallOpenDot",
        "DiamondWide",
        "DiamondWideOpen",
        "DiamondWideDot",
        "DiamondWideOpenDot",
        "Hourglass",
        "HourglassOpen",
        "BowTie",
        "BowTieOpen",
        "CircleCross",
        "CircleCrossOpen",
        "CircleX",
        "CircleXOpen",
        "SquareCross",
        "SquareCrossOpen",
        "SquareX",
        "SquareXOpen",
        "DiamondCross",
        "DiamondCrossOpen",
        "DiamondX",
        "DiamondXOpen",
        "CrossThin",
        "CrossThinOpen",
        "XThin",
        "XThinOpen",
        "Asterisk",
        "AsteriskOpen",
        "Hash",
        "HashOpen",
        "HashDot",
        "HashOpenDot",
        "YUp",
        "YUpOpen",
        "YDown",
        "YDownOpen",
        "YLeft",
        "YLeftOpen",
        "YRight",
        "YRightOpen",
        "LineEW",
        "LineEWOpen",
        "LineNS",
        "LineNSOpen",
        "LineNE",
        "LineNEOpen",
        "LineNW",
        "LineNWOpen",
    ];
    let mut rng = rand::thread_rng();
    let mut ret: Vec<MarkerSymbol> = Vec::with_capacity(n);
    for _ in 0..n {
        let symbol = pool[rng.gen_range(0..25)];
        let marker = match symbol {
            "Circle" => MarkerSymbol::Circle,
            "CircleOpen" => MarkerSymbol::CircleOpen,
            "CircleDot" => MarkerSymbol::CircleDot,
            "CircleOpenDot" => MarkerSymbol::CircleOpenDot,
            "Square" => MarkerSymbol::Square,
            "SquareDot" => MarkerSymbol::SquareDot,
            "SquareOpen" => MarkerSymbol::SquareOpen,
            "SquareOpenDot" => MarkerSymbol::SquareOpenDot,
            "Diamond" => MarkerSymbol::Diamond,
            "DiamondOpen" => MarkerSymbol::DiamondOpen,
            "DiamondDot" => MarkerSymbol::DiamondDot,
            "DiamondOpenDot" => MarkerSymbol::DiamondOpenDot,
            "Hash" => MarkerSymbol::Hash,
            "HashDot" => MarkerSymbol::HashDot,
            "HashOpen" => MarkerSymbol::HashOpen,
            "HashOpenDot" => MarkerSymbol::HashOpenDot,
            "Cross" => MarkerSymbol::Cross,
            "CrossDot" => MarkerSymbol::CrossDot,
            "CrossOpen" => MarkerSymbol::CrossOpen,
            "CrossOpenDot" => MarkerSymbol::CrossOpenDot,
            "TriangleUp" => MarkerSymbol::TriangleUp,
            "TriangleUpDot" => MarkerSymbol::TriangleUpDot,
            "TriangleUpOpen" => MarkerSymbol::TriangleUpOpen,
            "TriangleUpOpenDot" => MarkerSymbol::TriangleUpOpenDot,
            "TriangleDown" => MarkerSymbol::TriangleDown,
            "X" => MarkerSymbol::X,
            "XOpen" => MarkerSymbol::XOpen,
            "XDot" => MarkerSymbol::XDot,
            "XOpenDot" => MarkerSymbol::XOpenDot,
            "YUp" => MarkerSymbol::YUp,
            "YUpOpen" => MarkerSymbol::YUpOpen,
            "YDown" => MarkerSymbol::YDown,
            "YDownOpen" => MarkerSymbol::YDownOpen,
            _ => MarkerSymbol::Cross,
        };
        ret.push(marker);
    }
    ret
}

/*
 * builds a standard 2D plot single Y scale,
 * ready to plot data against time (`Epoch`)
 */
pub fn build_default_plot(title: &str, y_title: &str) -> Plot {
    build_plot(
        title,
        Side::Top,
        Font::default(),
        "Epoch",
        y_title,
        (true, true), // y=0 lines
        true,         // show legend
        true,         // autosize
    )
}

/*
 * build a standard 2D plot dual Y axes,
 * to plot against `Epochs`
 */
pub fn build_default_2y_plot(title: &str, y1_title: &str, y2_title: &str) -> Plot {
    build_plot_2y(
        title,
        Side::Top,
        Font::default(),
        "Epoch",
        y1_title,
        y2_title,
        (false, false), // y=0 lines
        true,           // show legend
        true,           // autosize
    )
}

/*
 * Builds a Plot
 */
fn build_plot(
    title: &str,
    title_side: Side,
    title_font: Font,
    x_axis_title: &str,
    y_axis_title: &str,
    zero_line: (bool, bool), // plots a bold line @ (x=0,y=0)
    show_legend: bool,
    auto_size: bool,
) -> Plot {
    let layout = Layout::new()
        .title(Title::new(title).font(title_font))
        .x_axis(
            Axis::new()
                .title(Title::new(x_axis_title).side(title_side))
                .zero_line(zero_line.0)
                .show_tick_labels(false),
        )
        .y_axis(
            Axis::new()
                .title(Title::new(y_axis_title))
                .zero_line(zero_line.0),
        )
        .show_legend(show_legend)
        .auto_size(auto_size);
    let mut p = Plot::new();
    p.set_layout(layout);
    p
}

fn build_plot_2y(
    title: &str,
    title_side: Side,
    title_font: Font,
    x_title: &str,
    y1_title: &str,
    y2_title: &str,
    zero_line: (bool, bool), // plots a bold line @ (x=0,y=0)
    show_legend: bool,
    auto_size: bool,
) -> Plot {
    let layout = Layout::new()
        .title(Title::new(title).font(title_font))
        .x_axis(
            Axis::new()
                .title(Title::new(x_title).side(title_side))
                .zero_line(zero_line.0)
                .show_tick_labels(false),
        )
        .y_axis(
            Axis::new()
                .title(Title::new(y1_title))
                .zero_line(zero_line.1),
        )
        .y_axis2(
            Axis::new()
                .title(Title::new(y2_title))
                .overlaying("y")
                .side(AxisSide::Right)
                .zero_line(zero_line.1),
        )
        .show_legend(show_legend)
        .auto_size(auto_size);
    let mut p = Plot::new();
    p.set_layout(layout);
    p
}

/*
 * Builds a default chart, 2D, X = time axis
 */
pub fn build_chart_epoch_axis<T: Clone + Default + Serialize>(
    name: &str,
    mode: Mode,
    epochs: Vec<Epoch>,
    data_y: Vec<T>,
) -> Box<Scatter<f64, T>> {
    let txt: Vec<String> = epochs.iter().map(|e| e.to_string()).collect();
    Scatter::new(epochs.iter().map(|e| e.to_utc_seconds()).collect(), data_y)
        .mode(mode)
        //.web_gl_mode(true)
        .name(name)
        .hover_text_array(txt)
        .hover_info(HoverInfo::All)
}
