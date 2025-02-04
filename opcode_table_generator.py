
# From https://www.nesdev.org/wiki/Visual6502wiki/6502_all_256_Opcodes
OPCODE_TABLE = """
 00 BRK 7        $00: bytes: 0 cycles: 0 _____=>_____ __ 
 01 ORA izx 6    $01: bytes: 2 cycles: 6 A____=>____P R_ izx
 02 *KIL         $02: CRASH
 03 *SLO izx 8   $03: bytes: 2 cycles: 8 A____=>____P RW izx
 04 *NOP zp 3    $04: bytes: 2 cycles: 3 _____=>_____ R_ zp
 05 ORA zp 3     $05: bytes: 2 cycles: 3 A____=>A___P R_ zp
 06 ASL zp 5     $06: bytes: 2 cycles: 5 _____=>____P RW zp
 07 *SLO zp 5    $07: bytes: 2 cycles: 5 A____=>A___P RW zp
 08 PHP 3        $08: bytes: 1 cycles: 3 ___SP=>___S_ _W 
 09 ORA imm 2    $09: bytes: 2 cycles: 2 _____=>A___P __ 
 0A ASL 2        $0A: bytes: 1 cycles: 2 A____=>A___P __ 
 0B *ANC imm 2   $0B: bytes: 2 cycles: 2 A____=>____P __ 
 0C *NOP abs 4   $0C: bytes: 3 cycles: 4 _____=>_____ R_ abs
 0D ORA abs 4    $0D: bytes: 3 cycles: 4 A____=>A___P R_ abs
 0E ASL abs 6    $0E: bytes: 3 cycles: 6 _____=>____P RW abs
 0F *SLO abs 6   $0F: bytes: 3 cycles: 6 A____=>A___P RW abs
 10 BPL rel 2*   $10: bytes: 2 cycles: 3 ____P=>_____ __ 
 11 ORA izy 5*   $11: bytes: 2 cycles: 5 A____=>____P R_ izy
 12 *KIL         $12: CRASH
 13 *SLO izy 8   $13: bytes: 2 cycles: 8 A____=>____P RW izy
 14 *NOP zpx 4   $14: bytes: 2 cycles: 4 _____=>_____ R_ zpx
 15 ORA zpx 4    $15: bytes: 2 cycles: 4 A____=>A___P R_ zpx
 16 ASL zpx 6    $16: bytes: 2 cycles: 6 _____=>____P RW zpx
 17 *SLO zpx 6   $17: bytes: 2 cycles: 6 A____=>A___P RW zpx
 18 CLC 2        $18: bytes: 1 cycles: 2 _____=>____P __ 
 19 ORA aby 4*   $19: bytes: 3 cycles: 4 A____=>A___P R_ absy
 1A *NOP 2       $1A: bytes: 1 cycles: 2 _____=>_____ __ 
 1B *SLO aby 7   $1B: bytes: 3 cycles: 7 A____=>A___P RW absy
 1C *NOP abx 4*  $1C: bytes: 3 cycles: 4 _____=>_____ R_ absx
 1D ORA abx 4*   $1D: bytes: 3 cycles: 4 A____=>A___P R_ absx
 1E ASL abx 7    $1E: bytes: 3 cycles: 7 _____=>____P RW absx
 1F *SLO abx 7   $1F: bytes: 3 cycles: 7 A____=>A___P RW absx
 20 JSR abs 6    $20: bytes: X cycles: 6 ___S_=>___S_ _W 
 21 AND izx 6    $21: bytes: 2 cycles: 6 _____=>A___P R_ izx
 22 *KIL         $22: CRASH
 23 *RLA izx 8   $23: bytes: 2 cycles: 8 ____P=>A___P RW izx
 24 BIT zp 3     $24: bytes: 2 cycles: 3 A____=>____P R_ zp
 25 AND zp 3     $25: bytes: 2 cycles: 3 A____=>A___P R_ zp
 26 ROL zp 5     $26: bytes: 2 cycles: 5 ____P=>____P RW zp
 27 *RLA zp 5    $27: bytes: 2 cycles: 5 A___P=>A___P RW zp
 28 PLP 4        $28: bytes: 1 cycles: 4 ___S_=>___SP __ 
 29 AND imm 2    $29: bytes: 2 cycles: 2 A____=>A___P __ 
 2A ROL 2        $2A: bytes: 1 cycles: 2 A___P=>A___P __ 
 2B *ANC imm 2   $2B: bytes: 2 cycles: 2 A____=>____P __ 
 2C BIT abs 4    $2C: bytes: 3 cycles: 4 A____=>____P R_ abs
 2D AND abs 4    $2D: bytes: 3 cycles: 4 A____=>A___P R_ abs
 2E ROL abs 6    $2E: bytes: 3 cycles: 6 ____P=>____P RW abs
 2F *RLA abs 6   $2F: bytes: 3 cycles: 6 A___P=>A___P RW abs
 30 BMI rel 2*   $30: bytes: 2 cycles: 2 _____=>_____ __ 
 31 AND izy 5*   $31: bytes: 2 cycles: 5 _____=>A___P R_ izy
 32 *KIL         $32: CRASH
 33 *RLA izy 8   $33: bytes: 2 cycles: 8 ____P=>A___P RW izy
 34 *NOP zpx 4   $34: bytes: 2 cycles: 4 _____=>_____ R_ zpx
 35 AND zpx 4    $35: bytes: 2 cycles: 4 A____=>A___P R_ zpx
 36 ROL zpx 6    $36: bytes: 2 cycles: 6 ____P=>____P RW zpx
 37 *RLA zpx 6   $37: bytes: 2 cycles: 6 A___P=>A___P RW zpx
 38 SEC 2        $38: bytes: 1 cycles: 2 _____=>____P __ 
 39 AND aby 4*   $39: bytes: 3 cycles: 4 A____=>A___P R_ absy
 3A *NOP 2       $3A: bytes: 1 cycles: 2 _____=>_____ __ 
 3B *RLA aby 7   $3B: bytes: 3 cycles: 7 A___P=>A___P RW absy
 3C *NOP abx 4*  $3C: bytes: 3 cycles: 4 _____=>_____ R_ absx
 3D AND abx 4*   $3D: bytes: 3 cycles: 4 A____=>A___P R_ absx
 3E ROL abx 7    $3E: bytes: 3 cycles: 7 ____P=>____P RW absx
 3F *RLA abx 7   $3F: bytes: 3 cycles: 7 A___P=>A___P RW absx
 40 RTI 6        $40: bytes: X cycles: 6 ___S_=>___SP __ 
 41 EOR izx 6    $41: bytes: 2 cycles: 6 A____=>____P R_ izx
 42 *KIL         $42: CRASH
 43 *SRE izx 8   $43: bytes: 2 cycles: 8 A____=>____P RW izx
 44 *NOP zp 3    $44: bytes: 2 cycles: 3 _____=>_____ R_ zp
 45 EOR zp 3     $45: bytes: 2 cycles: 3 A____=>A___P R_ zp
 46 LSR zp 5     $46: bytes: 2 cycles: 5 _____=>____P RW zp
 47 *SRE zp 5    $47: bytes: 2 cycles: 5 A____=>A___P RW zp
 48 PHA 3        $48: bytes: 1 cycles: 3 A__S_=>___S_ _W 
 49 EOR imm 2    $49: bytes: 2 cycles: 2 A____=>A___P __ 
 4A LSR 2        $4A: bytes: 1 cycles: 2 A____=>A___P __ 
 4B *ALR imm 2   $4B: bytes: 2 cycles: 2 A____=>A___P __ 
 4C JMP abs 3    $4C: bytes: X cycles: 3 _____=>_____ __ 
 4D EOR abs 4    $4D: bytes: 3 cycles: 4 A____=>A___P R_ abs
 4E LSR abs 6    $4E: bytes: 3 cycles: 6 _____=>____P RW abs
 4F *SRE abs 6   $4F: bytes: 3 cycles: 6 A____=>A___P RW abs
 50 BVC rel 2*   $50: bytes: 2 cycles: 3 ____P=>_____ __ 
 51 EOR izy 5*   $51: bytes: 2 cycles: 5 A____=>____P R_ izy
 52 *KIL         $52: CRASH
 53 *SRE izy 8   $53: bytes: 2 cycles: 8 A____=>____P RW izy
 54 *NOP zpx 4   $54: bytes: 2 cycles: 4 _____=>_____ R_ zpx
 55 EOR zpx 4    $55: bytes: 2 cycles: 4 A____=>A___P R_ zpx
 56 LSR zpx 6    $56: bytes: 2 cycles: 6 _____=>____P RW zpx
 57 *SRE zpx 6   $57: bytes: 2 cycles: 6 A____=>A___P RW zpx
 58 CLI 2        $58: bytes: 1 cycles: 2 _____=>____P __ 
 59 EOR aby 4*   $59: bytes: 3 cycles: 4 A____=>A___P R_ absy
 5A *NOP 2       $5A: bytes: 1 cycles: 2 _____=>_____ __ 
 5B *SRE aby 7   $5B: bytes: 3 cycles: 7 A____=>A___P RW absy
 5C *NOP abx 4*  $5C: bytes: 3 cycles: 4 _____=>_____ R_ absx
 5D EOR abx 4*   $5D: bytes: 3 cycles: 4 A____=>A___P R_ absx
 5E LSR abx 7    $5E: bytes: 3 cycles: 7 _____=>____P RW absx
 5F *SRE abx 7   $5F: bytes: 3 cycles: 7 A____=>A___P RW absx
 60 RTS 6        $60: bytes: X cycles: 6 ___S_=>___S_ __ 
 61 ADC izx 6    $61: bytes: 2 cycles: 6 A___P=>A___P R_ izx
 62 *KIL         $62: CRASH
 63 *RRA izx 8   $63: bytes: 2 cycles: 8 A___P=>A___P RW izx
 64 *NOP zp 3    $64: bytes: 2 cycles: 3 _____=>_____ R_ zp
 65 ADC zp 3     $65: bytes: 2 cycles: 3 A___P=>A___P R_ zp
 66 ROR zp 5     $66: bytes: 2 cycles: 5 ____P=>____P RW zp
 67 *RRA zp 5    $67: bytes: 2 cycles: 5 A___P=>A___P RW zp
 68 PLA 4        $68: bytes: 1 cycles: 4 ___S_=>A__SP __ 
 69 ADC imm 2    $69: bytes: 2 cycles: 2 A___P=>A___P __ 
 6A ROR 2        $6A: bytes: 1 cycles: 2 A___P=>A___P __ 
 6B *ARR imm 2   $6B: bytes: 2 cycles: 2 A___P=>A___P __ 
 6C JMP ind 5    $6C: bytes: X cycles: 5 _____=>_____ __ 
 6D ADC abs 4    $6D: bytes: 3 cycles: 4 A___P=>A___P R_ abs
 6E ROR abs 6    $6E: bytes: 3 cycles: 6 ____P=>____P RW abs
 6F *RRA abs 6   $6F: bytes: 3 cycles: 6 A___P=>A___P RW abs
 70 BVS rel 2*   $70: bytes: 2 cycles: 2 _____=>_____ __ 
 71 ADC izy 5*   $71: bytes: 2 cycles: 5 A___P=>A___P R_ izy
 72 *KIL         $72: CRASH
 73 *RRA izy 8   $73: bytes: 2 cycles: 8 A___P=>A___P RW izy
 74 *NOP zpx 4   $74: bytes: 2 cycles: 4 _____=>_____ R_ zpx
 75 ADC zpx 4    $75: bytes: 2 cycles: 4 A___P=>A___P R_ zpx
 76 ROR zpx 6    $76: bytes: 2 cycles: 6 ____P=>____P RW zpx
 77 *RRA zpx 6   $77: bytes: 2 cycles: 6 A___P=>A___P RW zpx
 78 SEI 2        $78: bytes: 1 cycles: 2 _____=>____P __ 
 79 ADC aby 4*   $79: bytes: 3 cycles: 4 A___P=>A___P R_ absy
 7A *NOP 2       $7A: bytes: 1 cycles: 2 _____=>_____ __ 
 7B *RRA aby 7   $7B: bytes: 3 cycles: 7 A___P=>A___P RW absy
 7C *NOP abx 4*  $7C: bytes: 3 cycles: 4 _____=>_____ R_ absx
 7D ADC abx 4*   $7D: bytes: 3 cycles: 4 A___P=>A___P R_ absx
 7E ROR abx 7    $7E: bytes: 3 cycles: 7 ____P=>____P RW absx
 7F *RRA abx 7   $7F: bytes: 3 cycles: 7 A___P=>A___P RW absx
 80 *NOP imm 2   $80: bytes: 2 cycles: 2 _____=>_____ __ 
 81 STA izx 6    $81: bytes: 2 cycles: 6 A____=>_____ RW izx
 82 *NOP imm 2   $82: bytes: 2 cycles: 2 _____=>_____ __ 
 83 *SAX izx 6   $83: bytes: 2 cycles: 6 _____=>_____ RW izx
 84 STY zp 3     $84: bytes: 2 cycles: 3 __Y__=>_____ _W zp
 85 STA zp 3     $85: bytes: 2 cycles: 3 A____=>_____ _W zp
 86 STX zp 3     $86: bytes: 2 cycles: 3 _X___=>_____ _W zp
 87 *SAX zp 3    $87: bytes: 2 cycles: 3 _____=>_____ _W zp
 88 DEY 2        $88: bytes: 1 cycles: 2 __Y__=>__Y_P __ 
 89 *NOP imm 2   $89: bytes: 2 cycles: 2 _____=>_____ __ 
 8A TXA 2        $8A: bytes: 1 cycles: 2 _X___=>A___P __ 
 8B *XAA imm 2   $8B: bytes: 2 cycles: 2 _____=>A___P __ 
 8C STY abs 4    $8C: bytes: 3 cycles: 4 __Y__=>_____ _W abs
 8D STA abs 4    $8D: bytes: 3 cycles: 4 A____=>_____ _W abs
 8E STX abs 4    $8E: bytes: 3 cycles: 4 _X___=>_____ _W abs
 8F *SAX abs 4   $8F: bytes: 3 cycles: 4 _____=>_____ _W abs
 90 BCC rel 2*   $90: bytes: 2 cycles: 3 ____P=>_____ __ 
 91 STA izy 6    $91: bytes: 2 cycles: 6 A____=>_____ RW izy
 92 *KIL         $92: CRASH
 93 *AHX izy 6   $93: bytes: 2 cycles: 6 _____=>_____ RW izy
 94 STY zpx 4    $94: bytes: 2 cycles: 4 __Y__=>_____ RW zpx
 95 STA zpx 4    $95: bytes: 2 cycles: 4 A____=>_____ RW zpx
 96 STX zpy 4    $96: bytes: 2 cycles: 4 _X___=>_____ RW zpy
 97 *SAX zpy 4   $97: bytes: 2 cycles: 4 _____=>_____ RW zpy
 98 TYA 2        $98: bytes: 1 cycles: 2 __Y__=>A___P __ 
 99 STA aby 5    $99: bytes: 3 cycles: 5 A____=>_____ RW absy
 9A TXS 2        $9A: bytes: X cycles: 2 _X___=>___S_ __ 
 9B *TAS aby 5   $9B: bytes: X cycles: 5 __Y__=>___S_ _W 
 9C *SHY abx 5   $9C: bytes: 3 cycles: 5 __Y__=>_____ RW absx
 9D STA abx 5    $9D: bytes: 3 cycles: 5 A____=>_____ RW absx
 9E *SHX aby 5   $9E: bytes: 3 cycles: 5 _X___=>_____ RW absy
 9F *AHX aby 5   $9F: bytes: 3 cycles: 5 _____=>_____ RW absy
 A0 LDY imm 2    $A0: bytes: 2 cycles: 2 _____=>__Y_P __ 
 A1 LDA izx 6    $A1: bytes: 2 cycles: 6 _____=>A___P R_ izx
 A2 LDX imm 2    $A2: bytes: 2 cycles: 2 _____=>_X__P __ 
 A3 *LAX izx 6   $A3: bytes: 2 cycles: 6 _____=>AX__P R_ izx
 A4 LDY zp 3     $A4: bytes: 2 cycles: 3 _____=>__Y_P R_ zp
 A5 LDA zp 3     $A5: bytes: 2 cycles: 3 _____=>A___P R_ zp
 A6 LDX zp 3     $A6: bytes: 2 cycles: 3 _____=>_X__P R_ zp
 A7 *LAX zp 3    $A7: bytes: 2 cycles: 3 _____=>AX__P R_ zp
 A8 TAY 2        $A8: bytes: 1 cycles: 2 A____=>__Y_P __ 
 A9 LDA imm 2    $A9: bytes: 2 cycles: 2 _____=>A___P __ 
 AA TAX 2        $AA: bytes: 1 cycles: 2 A____=>_X__P __ 
 AB *LAX imm 2   $AB: bytes: 2 cycles: 2 A____=>AX__P __ 
 AC LDY abs 4    $AC: bytes: 3 cycles: 4 _____=>__Y_P R_ abs
 AD LDA abs 4    $AD: bytes: 3 cycles: 4 _____=>A___P R_ abs
 AE LDX abs 4    $AE: bytes: 3 cycles: 4 _____=>_X__P R_ abs
 AF *LAX abs 4   $AF: bytes: 3 cycles: 4 _____=>AX__P R_ abs
 B0 BCS rel 2*   $B0: bytes: 2 cycles: 2 _____=>_____ __ 
 B1 LDA izy 5*   $B1: bytes: 2 cycles: 5 _____=>A___P R_ izy
 B2 *KIL         $B2: CRASH
 B3 *LAX izy 5*  $B3: bytes: 2 cycles: 5 _____=>AX__P R_ izy
 B4 LDY zpx 4    $B4: bytes: 2 cycles: 4 _____=>__Y_P R_ zpx
 B5 LDA zpx 4    $B5: bytes: 2 cycles: 4 _____=>A___P R_ zpx
 B6 LDX zpy 4    $B6: bytes: 2 cycles: 4 _____=>_X__P R_ zpy
 B7 *LAX zpy 4   $B7: bytes: 2 cycles: 4 _____=>AX__P R_ zpy
 B8 CLV 2        $B8: bytes: 1 cycles: 2 _____=>____P __ 
 B9 LDA aby 4*   $B9: bytes: 3 cycles: 4 _____=>A___P R_ absy
 BA TSX 2        $BA: bytes: 1 cycles: 2 ___S_=>_X__P __ 
 BB *LAS aby 4*  $BB: bytes: 3 cycles: 4 ___S_=>AX_SP R_ absy
 BC LDY abx 4*   $BC: bytes: 3 cycles: 4 _____=>__Y_P R_ absx
 BD LDA abx 4*   $BD: bytes: 3 cycles: 4 _____=>A___P R_ absx
 BE LDX aby 4*   $BE: bytes: 3 cycles: 4 _____=>_X__P R_ absy
 BF *LAX aby 4*  $BF: bytes: 3 cycles: 4 _____=>AX__P R_ absy
 C0 CPY imm 2    $C0: bytes: 2 cycles: 2 __Y__=>____P __ 
 C1 CMP izx 6    $C1: bytes: 2 cycles: 6 A____=>____P R_ izx
 C2 *NOP imm 2   $C2: bytes: 2 cycles: 2 _____=>_____ __ 
 C3 *DCP izx 8   $C3: bytes: 2 cycles: 8 A____=>____P RW izx
 C4 CPY zp 3     $C4: bytes: 2 cycles: 3 __Y__=>____P R_ zp
 C5 CMP zp 3     $C5: bytes: 2 cycles: 3 A____=>____P R_ zp
 C6 DEC zp 5     $C6: bytes: 2 cycles: 5 _____=>____P RW zp
 C7 *DCP zp 5    $C7: bytes: 2 cycles: 5 A____=>____P RW zp
 C8 INY 2        $C8: bytes: 1 cycles: 2 __Y__=>__Y_P __ 
 C9 CMP imm 2    $C9: bytes: 2 cycles: 2 A____=>____P __ 
 CA DEX 2        $CA: bytes: 1 cycles: 2 _X___=>_X__P __ 
 CB *AXS imm 2   $CB: bytes: 2 cycles: 2 _____=>_X__P __ 
 CC CPY abs 4    $CC: bytes: 3 cycles: 4 __Y__=>____P R_ abs
 CD CMP abs 4    $CD: bytes: 3 cycles: 4 A____=>____P R_ abs
 CE DEC abs 6    $CE: bytes: 3 cycles: 6 _____=>____P RW abs
 CF *DCP abs 6   $CF: bytes: 3 cycles: 6 A____=>____P RW abs
 D0 BNE rel 2*   $D0: bytes: 2 cycles: 3 ____P=>_____ __ 
 D1 CMP izy 5*   $D1: bytes: 2 cycles: 5 A____=>____P R_ izy
 D2 *KIL         $D2: CRASH
 D3 *DCP izy 8   $D3: bytes: 2 cycles: 8 A____=>____P RW izy
 D4 *NOP zpx 4   $D4: bytes: 2 cycles: 4 _____=>_____ R_ zpx
 D5 CMP zpx 4    $D5: bytes: 2 cycles: 4 A____=>____P R_ zpx
 D6 DEC zpx 6    $D6: bytes: 2 cycles: 6 _____=>____P RW zpx
 D7 *DCP zpx 6   $D7: bytes: 2 cycles: 6 A____=>____P RW zpx
 D8 CLD 2        $D8: bytes: 1 cycles: 2 _____=>____P __ 
 D9 CMP aby 4*   $D9: bytes: 3 cycles: 4 A____=>____P R_ absy
 DA *NOP 2       $DA: bytes: 1 cycles: 2 _____=>_____ __ 
 DB *DCP aby 7   $DB: bytes: 3 cycles: 7 A____=>____P RW absy
 DC *NOP abx 4*  $DC: bytes: 3 cycles: 4 _____=>_____ R_ absx
 DD CMP abx 4*   $DD: bytes: 3 cycles: 4 A____=>____P R_ absx
 DE DEC abx 7    $DE: bytes: 3 cycles: 7 _____=>____P RW absx
 DF *DCP abx 7   $DF: bytes: 3 cycles: 7 A____=>____P RW absx
 E0 CPX imm 2    $E0: bytes: 2 cycles: 2 _X___=>____P __ 
 E1 SBC izx 6    $E1: bytes: 2 cycles: 6 A___P=>A___P R_ izx
 E2 *NOP imm 2   $E2: bytes: 2 cycles: 2 _____=>_____ __ 
 E3 *ISC izx 8   $E3: bytes: 2 cycles: 8 A___P=>A___P RW izx
 E4 CPX zp 3     $E4: bytes: 2 cycles: 3 _X___=>____P R_ zp
 E5 SBC zp 3     $E5: bytes: 2 cycles: 3 A___P=>A___P R_ zp
 E6 INC zp 5     $E6: bytes: 2 cycles: 5 _____=>____P RW zp
 E7 *ISC zp 5    $E7: bytes: 2 cycles: 5 A___P=>A___P RW zp
 E8 INX 2        $E8: bytes: 1 cycles: 2 _X___=>_X__P __ 
 E9 SBC imm 2    $E9: bytes: 2 cycles: 2 A___P=>A___P __ 
 EA NOP 2        $EA: bytes: 1 cycles: 2 _____=>_____ __ 
 EB *SBC imm 2   $EB: bytes: 2 cycles: 2 A___P=>A___P __ 
 EC CPX abs 4    $EC: bytes: 3 cycles: 4 _X___=>____P R_ abs
 ED SBC abs 4    $ED: bytes: 3 cycles: 4 A___P=>A___P R_ abs
 EE INC abs 6    $EE: bytes: 3 cycles: 6 _____=>____P RW abs
 EF *ISC abs 6   $EF: bytes: 3 cycles: 6 A___P=>A___P RW abs
 F0 BEQ rel 2*   $F0: bytes: 2 cycles: 2 _____=>_____ __ 
 F1 SBC izy 5*   $F1: bytes: 2 cycles: 5 A___P=>A___P R_ izy
 F2 *KIL         $F2: CRASH
 F3 *ISC izy 8   $F3: bytes: 2 cycles: 8 A___P=>A___P RW izy
 F4 *NOP zpx 4   $F4: bytes: 2 cycles: 4 _____=>_____ R_ zpx
 F5 SBC zpx 4    $F5: bytes: 2 cycles: 4 A___P=>A___P R_ zpx
 F6 INC zpx 6    $F6: bytes: 2 cycles: 6 _____=>____P RW zpx
 F7 *ISC zpx 6   $F7: bytes: 2 cycles: 6 A___P=>A___P RW zpx
 F8 SED 2        $F8: bytes: 1 cycles: 2 _____=>____P __ 
 F9 SBC aby 4*   $F9: bytes: 3 cycles: 4 A___P=>A___P R_ absy
 FA *NOP 2       $FA: bytes: 1 cycles: 2 _____=>_____ __ 
 FB *ISC aby 7   $FB: bytes: 3 cycles: 7 A___P=>A___P RW absy
 FC *NOP abx 4*  $FC: bytes: 3 cycles: 4 _____=>_____ R_ absx
 FD SBC abx 4*   $FD: bytes: 3 cycles: 4 A___P=>A___P R_ absx
 FE INC abx 7    $FE: bytes: 3 cycles: 7 _____=>____P RW absx
 FF *ISC abx 7   $FF: bytes: 3 cycles: 7 A___P=>A___P RW absx
"""

def main():
    operations = set()
    address_func = {
        "imm" : "immediate",
        "zp" : "zero_page",
        "zpx" : "zero_page_x",
        "zpy" : "zero_page_y",
        "izx" : "indirect_x",
        "izy" : "indirect_y",
        "abs" : "absolute",
        "abx" : "absolute_x",
        "aby": "absolute_y",
        "ind": "indirect",
        "rel": "relative",
        "imp": "implied"
    }

    print("// Autogenerated from opcode_table_generator.py")
    print("static OPCODE_TABLE: [OpCode; 256] = [")
    for line in OPCODE_TABLE.split('\n'):
        line = line.strip()
        if line:
            opcode_info, _ = line.split('$')
            opcode_info = opcode_info.strip()
            opcode_info = opcode_info.split(' ')
            if len(opcode_info) == 3:
                # addressing mode is implied
                opcode,name,cycles = opcode_info
                addr = 'imp'
            elif len(opcode_info) == 4:
                opcode,name,addr,cycles = opcode_info
            name = name.replace('*', '')
            addr = addr.replace('*', '')
            cycles = cycles.replace('*', '')
            addressing = address_func[addr]
            operations.add(name.lower())
            print('// Opcode: 0x%s' % opcode)
            print('OpCode { execute: CPU::%s, addressing: CPU::%s, name: "%s", addr_name: "%s", cycles: %s },' % (name.lower(), addressing, name, addr.upper(), cycles))
    print("];")

    print("impl CPU {")
    for op_name in sorted(operations):
        print("fn %s(&mut self, _address: Address) {" % op_name)
        print('  todo!("%s Not Implemented")' % op_name)
        print('}')
        print()
    print("}")
if __name__ == '__main__':
    main()