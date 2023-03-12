use methods::{ MPT_PROOF_ID, MPT_PROOF_ELF };
use proof_core::{ ProofInput, ProofOutput, EthGetProofBody, EthGetBlockBody };
use prefix_hex::decode;
use sha3::{ Keccak256, Digest };
use serde::{ Deserialize, Serialize };
use serde_json::{ json, Value, Map };
use risc0_zkvm::{ Prover, Receipt, serde::to_vec, serde::from_slice };

use ureq;

pub fn run_prover(request: ProofInput) -> Receipt {
    let mut prover = Prover::new(MPT_PROOF_ELF, MPT_PROOF_ID).expect(
        "Prover should be constructed from valid method source code and corresponding image ID"
    );

    // Next we send input to the guest
    prover.add_input_u32_slice(to_vec(&request).expect("Input should be serializable").as_slice());

    let receipt = prover
        .run()
        .expect(
            "Code should be provable unless it had an error or overflowed the maximum cycle count"
        );

    receipt
}

pub fn get_input(
    provider: &str,
    address: &str,
    block_number: &str
) -> Result<ProofInput, ureq::Error> {
    let block_response = get_block_by_number(provider, block_number).unwrap();
    // for the proof block number, we pass whatever block the previous described to make sure
    // they are the same (e.g. if "latest" was used there could be a discrepancy)
    let proof_response = get_proof(provider, address, &block_response.number).unwrap();

    let result = ProofInput {
        root: block_response.state_root,
        account_proof: proof_response.account_proof,
        key: sha3::Keccak256::digest(proof_response.address).into(),
        expected_balance: 0,
    };

    Ok(result)
}

fn get_proof(
    provider: &str,
    address: &str,
    block_number: &str
) -> Result<EthGetProofBody, ureq::Error> {
    // Create an HTTP client
    let agent = ureq::agent();

    // eth_getProof POST request to the JSON-RPC provider, with the same block number
    let proof_response: Value = agent
        .post(provider)
        .send_json(
            ureq::json!(
                ureq::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_getProof",
                    "params": [address,
                        [],
                        block_number],
                })
            )
        )?
        .into_json()?;
    // Parse response as object
    let proof_response = proof_response["result"].as_object().unwrap();

    // Parse accountProof field to Vec<Vec<u8>>
    let account_proof_json = proof_response["accountProof"].as_array().unwrap();
    let account_proof = account_proof_json
        .iter()
        .map(|hex_string| decode(hex_string.as_str().unwrap()).unwrap())
        .collect::<Vec<Vec<u8>>>();

    // Parse address field as [u8; 20]
    let address = decode(&proof_response["address"].as_str().unwrap().to_owned()).unwrap();

    let proof_info = EthGetProofBody {
        address,
        account_proof,
    };

    Ok(proof_info)
}

fn get_block_by_number(provider: &str, block_number: &str) -> Result<EthGetBlockBody, ureq::Error> {
    // Create an HTTP client
    let agent = ureq::agent();

    // eth_getBlockByNumber POST request to the JSON-RPC provider
    let block_response: Value = agent
        .post(provider)
        .send_json(
            ureq::json!(    {"jsonrpc": "2.0",
           "id": 1,
           "method": "eth_getBlockByNumber",
           "params": [block_number, false]}
       )
        )?
        .into_json()?;
    // Parse response as object
    let block_response = block_response["result"].as_object().unwrap();
    let block_info = EthGetBlockBody {
        number: block_response["number"].as_str().unwrap().to_owned(),
        state_root: decode(&block_response["stateRoot"].as_str().unwrap().to_owned()).unwrap(),
    };

    Ok(block_info)
}

pub fn example_input() -> ProofInput {
    let root: [u8; 32] = decode(
        "0xf1428e97d77482a33d8a4e829c1b3aef8d7df63da7a6ce20aa3b32613099e8cb"
    ).unwrap();

    let key: [u8; 32] = Keccak256::digest(
        decode::<Vec<u8>>("0x904a81b8945803bacbb6c75ab4c956b173975954").unwrap()
    ).into();

    let account_proof: Vec<Vec<u8>> = vec![
        "0xf90211a021f62d917251f905b14ddcf004cc00cd3dc138d219ab7ec4065791553de8d2caa0953eed191019a5b64eb0604f25c0a78771eefbe132726e08b2296bd94855fd09a0ab8e62aec18b1ea7491d8b1efb4ea4720fa653569d0fb359352873d7af118af0a078c3df8ba743a46d0148456ddbb99c86af6ede8574b4e3717ce92ca54f79c1c7a0e6660aee178c47882ced0093e17f4b38cead251d1ce7e63c525cb79e0b401499a0774f04751aa541106a073d4ddb4b803e5b7a0e6470650857e33733c86171034ca0493642ddfe8ad6e825e4930d1fd9d4e9dbdb901f074c2e2b642b2c70f18b6687a063c26e040bac9f7d7317e4cc62934b6a0398e5124a4156ca9dfff735cb874163a0bcf6efcffea9dda6c69fa1454656f633f35ab248f0df299d64adebdc3140adf9a0af8a60cd3d98b3d8e8583c8d958a7cb539c3ad46c6ae0aacc1e44aeaf83b0bb7a05d78bffd2a3b7718df5819c92c0d36dd4e51163912e7c8aeec4e0ef46625b5b1a0103e02f0f74a3d35bbea856d4203eff54385d76f9e1fda0c4e9c90890c88ff5ba0e2a46830c578d64b62959baa933f8bf43e344b4848a059a95ac88766c19dbe91a00cf7839255578a0cdec01d60b4ad24585ac9adb239657a49e1e4b9262710776ca08a3d20c1dc0eb7b5caf6a632fa5dfa6b219c5e61e21ae7b441a405ec1327a720a098645b3ad6ce86d49f4d5334fe86d4f21cbc845e3bf346a3193c6a90fe8c313680",
        "0xf90211a0a37bfe8d9b562284abff4232ec0d1f8df37c34ad50469b6edff1af0eb9302c55a05a254094d2a78a5b8a36a675bc97125ead6db3875cbbba0e2b5eee40a61debf7a0130cb8ee63f3a53a1567e283b0b267b0db639e9a0caf52cd56308e6abd424137a0690b7b75d4137c329657e767a6f6776f57a6ba036948280ac036da7b3af6e12ca08be274fc5dcbacf5fbbe4945373db0dc4cfb12d78772f3a8b66e8d45dc264b2ba0ad61a3a1da2a5839d9618e1d668867b5ac71c937192be8b38054488ba5507d42a04b010b03debefd9bb3b0f28699734053511144b21aed1e69dd7ff5822e5404e6a022146b37dffa4d8b1d4eb9d5a20d38b9f105b20351eb56d0c07ab472af6092e5a0a6abd90226fea1da273d62b08b68a1c6802f28e0d550da8abf9d7fd2ac743337a02409e8f0ba74a2a5fa6fe1f861e4d25025fcc8f1ceec54625dd49b83d3af346ca0cb3511ba6ae2bcc147da5668eccc30ec1c07634132052ff361220a5faf297d4aa0126b050d015ec3acf8ffa67337e1e7e525c9dbfc66f18dabd69031c0fba0b44fa0c61ea8805ba6bd265256dbea09caac5fdf0cdce338c27a035d45bb6dea7a598fa0888f6c03e165d346a1a7db1cf42b3983ddcf21545167ffc9285dc9998b94b09aa06624c21c9abef40e74eaae2e8c41d0c31aaf773c51e51f1c474b16a97dd17e23a0613050dfcd6f079b3aa147c5f9b0fff4a56622f516c2ccade51776c68399dd9e80",
        "0xf90211a0e02ea271587ac4e31e1cf310d4a5243cdd2ba8e30a4cbba831eb6c6da13b3f35a04e69ab187d7115ccd62a9327dfe6f25b75a64f7c4f5d9387dd0cbf63e2c86ebba0592186cbf233d0919bcc9965261fa12ce349640143300df28078416f225ffcb0a085874c094740525ed0c3dc912ad10c3df1e309b61dfc391667761ada5a7341d5a0958c8a19ad0e9d9c68f59068adca7107871da2d64567a0fe3515837c56a64077a0024953bdc9d1fa28de069bbfcc65b8c5de01daacfdc8825e0a797bea826e49f4a01cf988885f2fa4dd2c59894b10b757ef29e4893d3c839752840851832dca91aba0562e1248e6ba71dcbb4f777f873705e71f194b4a22dc0f2e93703b379a4cdeb2a053caa7e4dc1f3e8d16976ee10ef78bac568f55af55f5932b97160576e688611da0ddb6e0fad00b80b8fb1ea247b8be433a3d1a01aa04a4be50845ec1e0380118d5a00883752a388219d5f862a4318630037b607054d72cf295f8efd4e2d69463b550a03e9f331a3c9712964283b66bfe49130bacc36a3b925a6e5c54641e641e7063f8a03a2c85bfc84a074a43ccfa89a72a6e7b7e5068c55105fbb3b109e0a2b9cc13a0a00b76bf1e6f39c3bc3eb9b929f20af6830f381ac3b707e857c8a2af9121585967a043b98c22ab5b1c346b66376b19b7e8f1899041cdfd218b61502a851a62e9a5c1a0718b5563fcce86e4ce87150577cbd40279a8f7695db79bdbedae9b350f73b7bb80",
        "0xf90211a00c286c4e1f8c1a89aadf49b3d57d460a8acc208ef2f66a4a83fe95f0409fdd1ba03188966bc1a338d90d6d9e81ce912d3e3e9603ce1d30f87089f943bfbfd37fd9a01f2f9f42b768e79bd23db656b4a45d5c7815abc329ba75d17a6fd20186383cfba0eeaf32eea99a116651b80f097b620959c9d9aacc66c92ed1bf1aef653e03df44a00dc66cc4647d662c92d41a21a219bd6345b0df663d19f00684ab97866b01076da00f90d34a2192da30ecc4af5de3516ddb188f4f52f3c70b4175463fe393a1f8aea02f29d2d385181858bc63d5e9c928ebe97b5456b3585c8de0e501807f0e420167a0fd10008f747d5c917171b91d307b1393f1dde5cdf1b1b44288fc00344bf8403da0e5fb1456cc598b0d75b3c41811139d722b52a20ec0d5b2635cd5bdd6fcb779d8a0cf9536abfbfa2e7b430d1df801f88ea206ca1b4a84555fa4f03d8e14653c4261a02b111db7f78bee3ab21c81f28697af9f7d2f5766fc38f16da3f44123aeb81823a06446bb02f6d1f7bdde1bfc1af151688e4925d94100d4138fd153835fbbe4f6baa04b9cb583029dfe32da1a88d9d7a28dca9a86722dbeb88b65ecf04dc5c8d49d97a0d0986f2e5199b9c62d60314ae86f2b0ca7f6fe06fb428a5c6f1e84fb5f3b4c00a08152d09a5fd6007891287eb418c26afa07f26f88ce95ebaf274a706f15159ae0a0e7d77956198b581455143e06c56b99eb141a77aa3203094c5bdc47247119250d80",
        "0xf90211a0966730b90fdf0c7e7939d6bea1a6c1ec86a58e2a93fd2d3747a4dc9d9ff51e41a0eb64e42d46c7c8bb6072565d034fea4c58d38afcf9b254a88963f553229be043a0d64dcc91ec5ef174495d06a89a27b80a5b1bed26f6755ef8a59adba367f72268a013fac6e2af8f7c76529aa9aa583ee07f1a3c7d774058b85fb8ff6ff30264fc10a089b4b1a8fe111bf4be9b6659c858dd0314ab770a16a8a14f614497480beb8466a0dcecb5ca3d0a908afc76e190e5e671a3c94d214450ca3e2405bc963a537ceed1a0247567f78a35c83279f7a358a9349dec52d165a8d266445e1efacb1ac686b2e6a04af6200aec2d49168c3ab45a50fec57b257aadb415da1e8986463a10e39122b7a0a98612c69e5b8e4b72c3ae72ae072ef051f33f11a3fe35023396a81d38a09d4ba0341e5cce7d13dab864e7d27b9419f5276e85e0216d157f59a87fadba286b6ec4a0c1d816150c609b6764e0b766e0b32409055558c51973a3b63198eb6636d2fbaba0ddf787f9222e3c04bc734a65cf4f0aac455cd4dbe943a15245a759a6d173efe2a0df90a5fcdebe875abbdbdfca9ed723de7821ba2f4b672ab727767d8fe346501ca0a0ce6e17c65cca59a29db5f91864aabd1ccf3a56d633b8abccbf3d5f72c0b7d7a0a4d42fb1baff4e9cd3fa78e23db95b317369264f5905bbf797ca6bea7f60e5b0a08e0b97809612f6d23f48f82c16d44f00390619048e46bb4408cf4e34974ff56f80",
        "0xf90211a06591d6675d0815033f39d3c9c4bb594bfb4b66cd9b63bfc91482850efb540552a03e934c158bd865d40ed00392a2cc7a1c5dc58ab3932ab3658aef3d0c9b49b9e0a060591368f229dd132749285c1c381cf858a83024eb509782139edd0f8481356ea08d3522847b90b326b507c2f30dff5fef4742444ac57c2d8925e00394ffb92c6fa0aeb24b25c8102093cf3a15bb87392bfde43f26c24c61c0cd112d5b95a242d86ba0e38b559f186f0bbb51ec1607fb77dfd3ced8a5546ca387096b22668449431926a082509ec27fcbda015a142e9760c76c4d6f70d057c413dde21d0ba7c2535c07fca024b47c31eb709f8d6c17bba5c6252fe7aa99d0813da1c61e0a1c00290c4ff6bfa0bc1b05cf76bad6b3b3c999ca8d1a6a2d07f0e79434061738338b00102ea751e7a0e90b493fe80040fe19b2136b38cf32525d0fcef64b6d0166c647baaa5ca00d8ca0e0c0ed1ce07ef53b1b42bd9f0c69bf37acff808a2e2a5a46eb5a219d80176b09a04ef793c229cd3f2f5d3a5ae75178b387c045da2b330b2d7477406b94c1e71e30a0f74410d2b7c3ba0bb032f28e01c74b09d3256386e7ec0d621839280927820d9da05351aff2bfc45aa9ea86930db2c5b6d4e6b828f0cd06e331d952e80686c405d1a0524dc6a5e5cf80ddf606554aef96ea4a6f5b9c7714f1a9688516cc701e8d60eda02d76f9fb9ef7fd9e76e65b11cfb759c3797164bf7cbc8ceae4dde6e41b64ef6980",
        "0xf90131a040bd2f3de78f7d5e19740cad63f83297176c282d725de89438223825f19cbf28808080a08a9d212cd8070370d671cca403ab00f88c56fd3c32ff478bdb4bc9dccbba1d67808080a0f8cdd71c37974b32330fc55de688177f73f45f3ddd11ff839458161b8c1544b8a0bc569096c6b39b6f2ec7544a129b62dc506bbf3c7c784461c94bebe13ea3b4a2a051d6f3f59885af6ec9e4ff5ba50df61752b11d6acbee6aadc5d564f2d36020e480a06f81c8e95d8f4320122d20669db23e505a8ba5c7a4969036364fe12333e14836a07db13d328f35c493453b8518f272b35f296547f4136cf56885abec218915bdd4a03c4d7f36c366507754b4867d60db95349e3e533667c017dfb09d1c51498a7217a0e716cd8c1fdbd476665a5c3a3414c65e4340a51b0ebe5c529dd97e168824d3d280",
        "0xf86e9d3e38043bcc09fc441f378c53cb98db7bb660d0939e0db8e4a9ec07d36ab84ef84c08881d278a153bc27aa1a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
    ]
        .iter()
        .map(|data| { decode(data).unwrap() })
        .collect();

    let request = ProofInput {
        root,
        account_proof,
        key,
        expected_balance: 100,
    };

    request
}