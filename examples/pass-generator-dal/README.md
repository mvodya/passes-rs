# DAL Boarding Pass Generator

Generator for Dodo Airlines Boarding pass

![Android & iOS Demo](demo.jpg)

## How to use

1. Put to `certs` directory certificates: `signerCert.pem` and `signerKey.key`.
2. Change [PassConfig](https://docs.rs/passes/latest/passes/struct.PassConfig.html) in main.rs
   1. Set `pass_type_identifier`
   2. Set `team_identifier`
3. Build with cargo & run
4. Open `DAL-boardingpass.pkpass` file