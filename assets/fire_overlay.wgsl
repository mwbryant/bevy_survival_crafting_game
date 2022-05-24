struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

let MAX_FIRES = 64;

struct Fire {
    position: vec2<f32>;
    strength: f32;
};

struct Fires {
    fires: array<Fire,MAX_FIRES>;
};

[[group(1), binding(0)]]
var<uniform> fires: Fires;

fn circle(st: vec2<f32>, center: vec2<f32>, radius: f32) -> f32{
    let dist = st-center;
    let smoothness = 1.5;
	return 1.0-smoothStep(radius-(radius*smoothness),
                         radius+(radius*smoothness),
                         dot(dist,dist)*4.0);
}
[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]]vec4<f32> {
    var color = vec4<f32>(0.0,0.0,0.0, 0.99);
    for( var i: i32 = 0; i < MAX_FIRES; i= i +1) {
        color = color * (1.0-circle(in.world_position.xy, fires.fires[i].position, fires.fires[i].strength ));
    }
    return color;
}