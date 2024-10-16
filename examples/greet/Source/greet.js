class Deserializer {
	source;
	offset;

	constructor(bytes) {
		this.source = bytes;
		this.offset = 0;
	}

	pop() {
		return this.source[this.offset++];
	}

	try_take_n(len) {
		const out = this.source.slice(this.offset, this.offset + len);
		this.offset += len;
		return out;
	}
}
function varint_max(type) {
	const BITS_PER_BYTE = 8;
	const BITS_PER_VARINT_BYTE = 7;

	const bits = type * BITS_PER_BYTE;

	const roundup_bits = bits + (BITS_PER_BYTE - 1);

	return Math.floor(roundup_bits / BITS_PER_VARINT_BYTE);
}
function max_of_last_byte(type) {
	let extra_bits = type % 7;
	return (1 << extra_bits) - 1;
}

function de_varint_big(de, type) {
	let out = 0n;

	for (let i = 0; i < varint_max(type); i++) {
		const val = de.pop();
		const carry = BigInt(val) & 0x7fn;
		out |= carry << (7n * BigInt(i));

		if ((val & 0x80) === 0) {
			if (i === varint_max(type) - 1 && val > max_of_last_byte(type)) {
				throw new Error("deserialize bad variant");
			} else {
				return out;
			}
		}
	}

	throw new Error("deserialize bad variant");
}

function de_varint(de, type) {
	let out = 0;

	for (let i = 0; i < varint_max(type); i++) {
		const val = de.pop();
		const carry = val & 0x7f;
		out |= carry << (7 * i);

		if ((val & 0x80) === 0) {
			if (i === varint_max(type) - 1 && val > max_of_last_byte(type)) {
				throw new Error("deserialize bad variant");
			} else {
				return out;
			}
		}
	}

	throw new Error("deserialize bad variant");
}
function deserializeU64(de) {
	return de_varint_big(de, 64);
}
function deserializeString(de) {
	const sz = deserializeU64(de);

	let bytes = de.try_take_n(Number(sz));

	return __text_decoder.decode(bytes);
}
function ser_varint(out, type, val) {
	let buf = [];
	for (let i = 0; i < varint_max(type); i++) {
		const buffer = new ArrayBuffer(type / 8);
		const view = new DataView(buffer);
		view.setInt16(0, val, true);
		buf[i] = view.getUint8(0);
		if (val < 128) {
			out.push(...buf);
			return;
		}

		buf[i] |= 0x80;
		val >>= 7;
	}
	out.push(...buf);
}

function ser_varint_big(out, type, val) {
	let buf = [];
	for (let i = 0; i < varint_max(type); i++) {
		const buffer = new ArrayBuffer(type / 8);
		const view = new DataView(buffer);
		view.setInt16(0, val, true);
		buf[i] = view.getUint8(0);
		if (val < 128) {
			out.push(...buf);
			return;
		}

		buf[i] |= 0x80;
		val >>= 7n;
	}
	out.push(...buf);
}
function serializeU64(out, val) {
	return ser_varint_big(out, 64, val);
}
function serializeString(out, val) {
	serializeU64(out, val.length);

	out.push(...__text_encoder.encode(val));
}
const __text_decoder = new TextDecoder("utf-8");
const __text_encoder = new TextEncoder();

/**
 * @param {string} myName
 * @returns {Promise<string>}
 */
export async function greet(myName) {
	const out = [];
	serializeString(out, myName);

	return fetch("ipc://localhost/greet/greet", {
		method: "POST",
		body: Uint8Array.from(out),
		headers: { "Content-Type": "application/octet-stream" },
	})
		.then((r) => r.arrayBuffer())
		.then((bytes) => {
			const de = new Deserializer(new Uint8Array(bytes));

			return deserializeString(de);
		});
}
