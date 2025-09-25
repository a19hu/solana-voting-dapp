import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair, PublicKey } from '@solana/web3.js'
import { Counter } from '../target/types/counter'
import { BankrunProvider, startAnchor } from "anchor-bankrun";

const IDL = require("../target/idl/counter.json");
const votingAddress = new PublicKey("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS")
describe('voting', () => {

  it('Initialize Poll', async () => {
    const context = await startAnchor("tests/anchor-example", [], []);

	const provider = new BankrunProvider(context);

  const votingProgram = new Program<Counter>(
		IDL,
		provider,
	);

  await votingProgram.methods
		.InitialzePoll(
      
    )
		.rpc();

  })
})
