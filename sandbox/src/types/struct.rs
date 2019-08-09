fn main() {
    // 名前つきフィールド構造体
    struct Polygon {
        vertexes: Vec<(i32, i32)>,
        stroke_width: u8,
        fill: (u8, u8, u8),
    }
    let triangle1 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 1,
    };

    // フィールド名でアクセス
    assert_eq!(triangle1.vertexes[0], (0, 0));
    assert_eq!(triangle1.vertexes.len(), 3);
    assert_eq!(triangle1.stroke_width, 1);
    assert_eq!(triangle1.fill, (255, 255, 255));

    let vertexes = vec![(0, 0), (3, 0), (2, 2)];
    let fill = (255, 255, 255);
    let stroke_width = 1;
    let mut triangle2 = Polygon {
        vertexes,
        fill,
        stroke_width,
    };
    let Polygon {
        vertexes: quad_vx, ..
    } = triangle2;
    assert_eq!(3, quad_vx.len());

    triangle2.fill = (0, 0, 0);
    let Polygon { fill, .. } = triangle2;
    assert_eq!((0, 0, 0), fill);

    // タプル構造体
    struct Triangle(Vertex, Vertex, Vertex);
    struct Vertex(i32, i32);

    let vx0 = Vertex(0, 0);
    let vx1 = Vertex(3, 0);
    let triangle = Triangle(vx0, vx1, Vertex(2, 2));

    assert_eq!((triangle.1).0, 3);
}
