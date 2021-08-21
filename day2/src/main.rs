// find out how many passwords are valid given password policy

fn main() -> anyhow::Result<()> {
    let s = include_str!("input.txt")
            .split('\n');

   dbg!(&s);
   Ok(())
}