fn main() {
    // let buffer: &mut [i32];
    // let coefficients: [i64; 12];
    // let qlp_shift: i16;

    // for i in 12..buffer.len() {
    //     let prediction = coefficients.iter()
    //                                 .zip(&buffer[i - 12..i])
    //                                 .map(|(&c, &s)| c * s as i64)
    //                                 .sum::<i64>() >> qlp_shift;
    //     let delta = buffer[i];
    //     buffer[i] = prediction as i32 + delta;
    // }

    // This solutions is made by chatGPT
    let mut buffer: Vec<i32> = vec![0; 100]; // Inicializa buffer con valores de ejemplo
    let coefficients: [i64; 12] = [1; 12]; // Inicializa coefficients con valores de ejemplo
    let qlp_shift: i16 = 2; // Inicializa qlp_shift con un valor de ejemplo

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = (prediction as i32 + delta) as i32;
    }

}
