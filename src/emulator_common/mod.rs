// Copyright 2017 Remi Bernotavicius

use serde_derive::{Deserialize, Serialize};

pub mod debugger;
pub mod disassembler;

/*  ___       _       _  ___   ___   ___   ___  ____            _     _
 * |_ _|_ __ | |_ ___| |( _ ) / _ \ ( _ ) / _ \|  _ \ ___  __ _(_)___| |_ ___ _ __
 *  | || '_ \| __/ _ \ |/ _ \| | | |/ _ \| | | | |_) / _ \/ _` | / __| __/ _ \ '__|
 *  | || | | | ||  __/ | (_) | |_| | (_) | |_| |  _ <  __/ (_| | \__ \ ||  __/ |
 * |___|_| |_|\__\___|_|\___/ \___/ \___/ \___/|_| \_\___|\__, |_|___/\__\___|_|
 *                                                        |___/
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intel8080Register {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    A = 6,

    // Conatins all of the condition bits.
    FLAGS = 7,

    // Stack Pointer (2 bytes)
    SP = 8,

    // Special fake register called 'Program Status Word'. It refers to register pair, A and
    // FLAGS.
    PSW = 10,

    // Special fake register called 'Memory'. Represents the data stored at address contained in
    // HL.
    M = 11,

    Count = 12,
}
