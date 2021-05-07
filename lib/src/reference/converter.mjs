import operations from "./operations.mjs";

function split_into_op_and_adr(operation) {
	let first = operation.type.toString().trim();
	if (first.endsWith("A") || first.endsWith("X") || first.endsWith("Y")) {
		first = first.substring(0, first.length - 1);
	}
	if (first.endsWith("CC") || first.endsWith("SP")) {
		first = first.substring(0, first.length - 2);
	}
	switch (operation.shortHand.substring(5).trim()) {
		case "":
			return [ first, "" ];
		case "n,X":
		case "n, X":
			return [ first, "Xn(b)" ];
		case "n,Y":
		case "n, Y":
			return [ first, "Yn(b)" ];
		case "Adr":
		case "ADR":
			return [ first, "Adr(b)" ];
		case "n, SP":
		case "n,SP":
			return [ first, "SP(b)" ];
		case "#Data":
			return [ first, "Data(b)" ];
		case "Y,SP":
		case "Y, SP":
			return [ first, "YSp" ];
		case "SP,Y":
		case "SP, Y":
			return [ first, "SpY" ];
		case "X,SP":
		case "X, SP":
			return [ first, "XSp" ];
		case "SP,X":
		case "SP, X":
			return [ first, "SpX" ];
		case "A, X":
		case "A,X":
			return [ first, "AX" ];
		case "A, Y":
		case "A,Y":
			return [ first, "AY" ];
		case "A, CC":
		case "A,CC":
			return [ first, "ACc" ];
		case "X,Y":
		case "X, Y":
			return [ first, "XY" ];
		case "AY":
		case "A,Y":
		case "A, Y":
			return [ first, "AY" ];
		case "AX":
		case "A,X":
		case "A, X":
			return [ first, "AX" ];
		case ",X+":
			return [ first, "Xp" ];
		case ",X-":
			return [ first, "Xm" ];
		case ",+X":
			return [ first, "pX" ];
		case ",-X":
			return [ first, "mX" ];
		case ",Y+":
			return [ first, "Yp" ];
		case ",Y-":
			return [ first, "Ym" ];
		case ",+Y":
			return [ first, "pY" ];
		case ",-Y":
			return [ first, "mY" ];
		case "CC,A":
		case "CC, A":
			return [ first, "CcA" ];
		case "Y,X":
		case "Y, X":
			return [ first, "YX" ];
		default:
			console.log(operation);
			throw "error";
	}
}

/*let result = operations.map((operation) => {
    let whatHow = split_into_op_and_adr(operation);
    return "0x" + operation.code + " => Instruction::" + whatHow[0] + "(Addressing::" + whatHow[1] +
           "),"
});*/

const result = operations.map(operation => {
	return "0x" + operation.code + " => Instruction::" + operation.shortHand + ",";
});

console.log("match a {");
for (let line of result) {
	console.log(line);
}
console.log("_ => return None,\n}");