import { Metadata } from "@metaplex-foundation/mpl-token-metadata";
import * as anchor from "@project-serum/anchor";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  mintTo,
} from "@solana/spl-token";
import { PublicKey, ParsedConfirmedTransaction } from "@solana/web3.js";

export const getInt64Bytes = async (x) => {
  let y = Math.floor(x / 2 ** 32);
  return [y, y << 8, y << 16, y << 24, x, x << 8, x << 16, x << 24].map(
    (z) => z >>> 24
  );
};

export const intFromBytes = (byteArr) => {
  return byteArr.reduce((a, c, i) => a + c * 2 ** (56 - i * 8), 0);
};

export const getPdaWithBump = async (program, seed, inputs) => {
  try {
    const [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode(seed),
        ...inputs.map((input) => input.toBytes()),
      ],
      program.programId
    );

    return [pda, bump];
  } catch (error) {
    console.log(error);
  }
};

function intToArray(i) {
  return Uint8Array.of(
    (i & 0xff000000) >> 24,
    (i & 0x00ff0000) >> 16,
    (i & 0x0000ff00) >> 8,
    (i & 0x000000ff) >> 0
  );
}

export const getDynamicPda = async (program, seed, pub, uint) => {
  const [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode(seed),
      pub.toBytes(),
      new anchor.BN(uint).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  return pda;
};

export const getPda = async (program, seed, inputs) => {
  const [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode(seed),
      ...inputs.map((input) => input.toBytes()),
    ],
    program.programId
  );

  return pda;
};

export const sleep = async (n) =>
  await new Promise((resolve) => setTimeout(resolve, n));

export const getAirdrop = async (provider, receiver) => {
  await provider.connection.confirmTransaction(
    await provider.connection.requestAirdrop(
      receiver,
      anchor.web3.LAMPORTS_PER_SOL * 2
    ),
    "confirmed"
  );
};
