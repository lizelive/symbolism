unicodeCharactersTRFileName = UsingFrontEnd[System`Dump`unicodeCharactersTR];
unicodeCharactersTR=Import[unicodeCharactersTRFileName,"TSV","HeaderLines" -> 1][[All,;;;;2]];

named[names_]:=p:Table[Blank[],Length[names]]:>AssociationThread[names,p]
makeParser[contract_]:=Rules[named/@contract]
contract={
{"make_boxes"},
{"character_code","alias","escape_sequences","kind"},
{"character_code","alias","escape_sequences","kind","u1","u2","u3"},
{"character_code","alias","escape_sequences","kind","u1","u2","u3","close_character_code"},
{"character_code","alias","escape_sequences","kind","u1","u2","u3","u4","close_character_code"}
};
rules=makeParser[contract];
data=rules/@unicodeCharactersTR;