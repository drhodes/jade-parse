use serde_json::{json, Value};

fn garr_inc_4_json() -> Value {
    // this json macro is slick.
    json!(["Jade", {"/user/GarrInc4":
                    {"properties":{"name":{"edit":"yes","type":"name","value":"","label":"Name"}},
                     "schematic":[["/gates/xor2",[152,128,1]],
                                  ["/gates/xor2",[80,128,1]],
                                  ["/gates/xor2",[8,128,1]],
                                  ["/gates/xor2",[-64,128,1]],
                                  ["wire", [-144,-88,0,0,24]],
                                  ["wire", [136,88,0,0,40]],
                                  ["wire", [80,128,0,0,-48]],
                                  ["wire", [128,88,0,8,0]],
                                  ["wire", [128,72,0,24,0]],
                                  ["wire", [152,128,0,0,-56]],
                                  ["wire", [8,128,0,0,-80]],
                                  ["wire", [56,64,0,8,0]],
                                  ["wire", [64,128,0,0,-64]],
                                  ["wire", [56,48,0,80,0]],
                                  ["wire", [136,88,0,0,-40]],
                                  ["wire", [56,32,0,96,0]],
                                  ["wire", [152,72,0,0,-40]],
                                  ["wire", [-64,128,0,0,-128]],
                                  ["wire", [-16,24,0,8,0]],
                                  ["wire", [-8,128,0,0,-104]],
                                  ["wire", [-16,8,0,80,0]],
                                  ["wire", [64,64,0,0,-56]],
                                  ["wire", [-16,-8,0,152,0]],
                                  ["wire", [136,48,0,0,-56]],
                                  ["wire", [-16,-24,0,168,0]],
                                  ["wire", [152,32,0,0,-56]],
                                  ["wire", [-96,-40,0,16,0]],
                                  ["wire", [-80,128,0,0,-168]],
                                  ["wire", [-80,-192,0,0,152],{"signal":"A[3]"}],
                                  ["wire", [-96,-56,0,88,0]],
                                  ["wire", [-8,24,0,0,-80]],
                                  ["wire", [-8,-192,0,0,136],{"signal":"A[2]"}],
                                  ["wire", [-96,-72,0,160,0]],
                                  ["wire", [64,8,0,0,-80]], 
                                  ["wire", [-96,-88,0,232,0]], 
                                  ["wire", [136,-192,0,0,104],{"signal":"A [0]"}],
                                  ["wire", [136,-8,0,0,-80]],
                                  ["wire", [152,-24,0,0,-80]],
                                  ["wire", [152,-104,0,32,0],{"signal":"Ci"}],
                                  ["wire", [-192,-96,0,-8,0],{"signal":"Co"}],
                                  ["wire", [64,-72,0,0,-120],{"signal":"A[1]"}],
                                  ["wire", [144,176,0,0,8],{"signal":"vout [0]"}],
                                  ["wire", [72,176,0,0,8],{"signal":"vout[1]"}],
                                  ["wire", [0,176,0,0,8],{"signal":"vout [2]"}],
                                  ["wire", [-72,176,0,0,8],{"signal":"vout[3]"}],
                                  ["wire", [-144,-104,0,296,0]],
                                  ["text", [-158,-230,0],{"text":"w/fast, delta_t = 1.777ns - 1.361ns  = -.416ns","font":"12pt sans-serif"}],
                                  ["/gates/and2", [128,72,4]],
                                  ["/gates/and3", [56,32,4]],
                                  ["/gates/and4", [-16,-24,4]],
                                  ["/gates/and4", [-96,-88,4]],
                                  ["/gates/and2", [-144,-104,4]]],
                     "icon": [["line", [-24,-24,0,48,0]],
                              ["line", [24,-24,0,0,48]],
                              ["line", [24,24,0,-48,0]],
                              ["line", [-24,24,0,0,-48]],
                              ["text", [-13,-9,0],{"text":"GarrInc"}],
                              ["terminal", [32,0,4],{"name":"Ci"}],
                              ["terminal", [-32,0,0],{"name":"Co"}],
                              ["terminal", [0,-32,1],{"name":"A[3:0]"}],
                              ["terminal", [0,32,3],{"name":"vout [3:0]"}],
                              ["text", [-8,-21,0],{"text":"A[3:0]","font":"4pt sans-serif"}],
                              ["text", [-22,0,0],{"text":"Co","font":"4pt sans-serif"}],
                              ["text", [17,0,0],{"text":"Ci","font":"4pt sans-serif"}],
                              ["text", [-10,20,0],{"text":"out[3:0]","font":"4pt sans-serif"}],
                              ["text", [-8,6,0],{"text":"4","font":"18pt sans-serif"}]],
                     "test": [["test", ".power Vdd=1\n.thresholds Vol=0 Vil=0.1 Vih=0.9 Voh=1\n\n.group inputs A[3:0] Ci\n.group outputs vout [3:0] Co\n\n.mode gate\n\n.cycle assert inputs tran 99n sample outputs tran 1n\n\n00000 LLLLL\n00001 LLLHL\n00010 LLLHL\n00100 LLHLL\n01000 LHLLL\n10000 HLLLL\n00011 LLHLL\n00101 LLHHL\n00110 LLHHL\n01001 LHLHL\n01010 LHLHL\n01100 LHHLL\n10001 HLLHL\n10010 HLLHL\n10100 HLHLL\n11000 HHLLL\n00111 LHLLL\n01011 LHHLL\n01101 LHHHL\n01110 LHHHL\n10011 HLHLL\n10101 HLHHL\n10110 HLHHL\n11001 HHLHL\n11010 HHLHL\n11100 HHHLL\n01111 HLLLL\n10111 HHLLL\n11011 HHHLL\n11101 HHHHL\n11110 HHHHL\n11111 LLLLH\n\n.plot D(Ci)\n.plot D(Co)\n.plot D(A[3:0])\n.plot D(vout[3:0])\n"]]}}])
}