pub fn interpreter(tape: &str, data: &str) -> String {
    let mut tape_read = tape.chars().cycle();
    data.chars()
        .map(|c| {
            assert!(c == '0' || c == '1', "Invalid data character `{}`", c);

            let doflip =
                tape_read
                    .by_ref()
                    .take_while(|t| *t != '0')
                    .fold(false, |doflip, t| match t {
                        '1' => !doflip,
                        '0' => doflip,
                        _ => panic!("Invalid tape character `{}`", t),
                    });
            if doflip {
                if c == '0' {
                    '1'
                } else {
                    '0'
                }
            } else {
                c
            }
        })
        .collect()
}
