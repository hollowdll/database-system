// Engine core library

#![allow(unused)]

struct Config {
    exit: bool,
}

impl Config {
    pub fn build() -> Config {


        return Config {
            exit: false,
        }
    }
}



#[cfg(test)]
mod tests {

}