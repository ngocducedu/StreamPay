import React, { useEffect, useState } from "react";
import { Button, Card, PageHeader, notification } from "antd";
import {
  SendOutlined,
  DollarCircleOutlined,
  CrownOutlined,
  StarFilled,
  PauseCircleOutlined,
  StopOutlined,
  FrownOutlined,
  RedoOutlined,
  ThunderboltFilled,
  BugOutlined,
  GroupOutlined,
} from "@ant-design/icons";
import ModalTransferNFT from "../components/ModalTransferNFT";
import ModalSale from "../components/ModalSale";
import ModalAuction from "../components/ModalAuction";
import { default as PublicKey, transactions, utils } from "near-api-js";
import { functionCall, createTransaction } from "near-api-js/lib/transaction";
import ModalMintNFT from "../components/ModelMintNFT";
import ModalMatingNFT from "../components/ModalMatingNFT";
import { login, parseTokenAmount } from "../utils";
import BN from "bn.js";
import { baseDecode } from "borsh";
import getConfig from "../config";
import { Progress, Row, Col } from "antd";
import { Link } from "react-router-dom";
import element0 from "../assets/element0.png";
import element1 from "../assets/element1.png";
import element2 from "../assets/element2.png";
import element3 from "../assets/element3.png";

const nearConfig = getConfig(process.env.NODE_ENV || "development");
const { Meta } = Card;

function StreamIncome() {
  const element = [element0, element1, element2, element3];
  const stars = [
    [
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled />,
      <StarFilled />,
      <StarFilled />,
      <StarFilled />,
    ],
    [
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled />,
      <StarFilled />,
      <StarFilled />,
    ],
    [
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled />,
      <StarFilled />,
    ],
    [
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled />,
    ],
    [
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
      <StarFilled style={{ color: "#ff9e0d" }} />,
    ],
  ];

  const hungry = [
    [
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
      <FrownOutlined style={{ color: "rgb(229 162 205)" }} />,
      <FrownOutlined style={{ color: "rgb(229 162 205)" }} />,
      <FrownOutlined style={{ color: "rgb(229 162 205)" }} />,
    ],
    [
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
      <FrownOutlined style={{ color: "rgb(229 162 205)" }} />,
      <FrownOutlined style={{ color: "rgb(229 162 205)" }} />,
    ],
    [
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
      <FrownOutlined style={{ color: "rgb(229 162 205)" }} />,
    ],
    [
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
      <FrownOutlined style={{ color: "rgb(225 14 149)" }} />,
    ],
  ];

  const [nfts, setNFTs] = useState([]);
  const [balanceWrap, setBalanceWrap] = useState(0);
  const [transferVisible, setTransferVisible] = useState(false);
  const [saleVisible, setSaleVisible] = useState(false);
  const [auctionVisible, setAuctionVisible] = useState(false);
  const [mintVisible, setMintVisible] = useState(false);
  const [currentToken, setCurrentToken] = useState(null);
  const [totalTicket, setTotalTicket] = useState(0);
  const [fundRaised, setFundRaised] = useState(0);
  const [streamAccount, setStreamAccount] = useState([]);
  const [dataStream, setDataStream] = useState([]);

  useEffect(async () => {
    let balanceWrap = await window.ftContract.ft_balance_of({
      account_id: window.accountId,
    });
    setBalanceWrap(balanceWrap);
  }, []);

  useEffect(async () => {
    if (window.accountId) {
      let streamdata = await window.streamContract.get_account_incoming_streams(
        { account_id: window.accountId, from: 0, limit: 30 }
      );
      setDataStream(streamdata);
      console.log("datastream : ", streamdata);
    }
  }, []);

  useEffect(async () => {
    let totalTicket = await window.contractNFT.nft_total_supply();
    setTotalTicket(totalTicket);
  }, []);

  useEffect(async () => {
    let fundRaised = await window.accountTree.getAccountBalance();
    setFundRaised(fundRaised);
  }, []);

  function getGas(gas) {
    return gas ? new BN(gas) : new BN("100000000000000");
  }
  function getAmount(amount) {
    return amount ? new BN(utils.format.parseNearAmount(amount)) : new BN("0");
  }

  async function handleWithdraw(id) {
    if (window.walletConnection.isSignedIn()) {
      await window.streamContract.withdraw(
        {
          stream_ids: [id],
        },
        200000000000000,
        1
      );
    } else {
      login();
    }
  }

  async function submitTransfer(accountId, tokenId) {
    try {
      if (accountId && currentToken.token_id) {
        await window.contractNFT.nft_transfer(
          {
            receiver_id: accountId,
            token_id: currentToken.token_id,
            approval_id: 0,
            memo: "Transfer to " + accountId,
          },
          30000000000000,
          1
        );
        setTransferVisible(false);
      }
    } catch (e) {
      console.log("Transfer error: ", e);
      setTransferVisible(false);
    } finally {
      setTransferVisible(false);
    }
  }

  async function submitOnAuction(token, startPrice, startTime, endTime) {
    try {
      if (startPrice && currentToken.token_id) {
        let auction_conditions =
          token === "NEAR"
            ? {
                is_native: true,
                contract_id: "near",
                decimals: "24",
                start_price: utils.format.parseNearAmount(
                  startPrice.toString()
                ),
                start_time: startTime.toString(),
                end_time: endTime.toString(),
              }
            : {
                is_native: false,
                contract_id: window.contractFT.contractId,
                decimals: "18",
                amount: parseTokenAmount(price, 18).toLocaleString("fullwide", {
                  useGrouping: false,
                }),
              };

        // Check storage balance
        let storageAccount = await window.contractMarket.storage_balance_of({
          account_id: window.accountId,
        });

        // Submit auction
        if (storageAccount > 0) {
          console.log(
            "Data: ",
            storageAccount,
            utils.format.parseNearAmount("0.1"),
            nearConfig.marketContractName
          );
          await window.contractNFT.nft_auction_approve(
            {
              token_id: currentToken.token_id,
              account_id: nearConfig.marketContractName,
              msg: JSON.stringify({ auction_conditions }),
            },
            30000000000000,
            utils.format.parseNearAmount("0.01")
          );
          setAuctionVisible(false);
        } else {
          notification["warning"]({
            message: "Not enough Storage Balance",
            description:
              "Your Storage Balance is not enough to sell on a new NFT. Please refill!",
          });
        }
      }
    } catch (e) {
      console.log("Transfer error: ", e);
      setTransferVisible(false);
    } finally {
      setTransferVisible(false);
    }
  }

  async function submitOnSale(token, price) {
    try {
      if (price && currentToken.token_id) {
        let sale_conditions =
          token === "NEAR"
            ? {
                is_native: true,
                contract_id: "near",
                decimals: "24",
                amount: utils.format.parseNearAmount(price.toString()),
              }
            : {
                is_native: false,
                contract_id: window.contractFT.contractId,
                decimals: "18",
                amount: parseTokenAmount(price, 18).toLocaleString("fullwide", {
                  useGrouping: false,
                }),
              };

        // Check storage balance
        let storageAccount = await window.contractMarket.storage_balance_of({
          account_id: window.accountId,
        });

        // Submit sale
        if (storageAccount > 0) {
          console.log(
            "Data: ",
            storageAccount,
            utils.format.parseNearAmount("0.1"),
            nearConfig.marketContractName
          );
          await window.contractNFT.nft_approve(
            {
              token_id: currentToken.token_id,
              account_id: nearConfig.marketContractName,
              msg: JSON.stringify({ sale_conditions }),
            },
            30000000000000,
            utils.format.parseNearAmount("0.01")
          );
          setSaleVisible(false);
        } else {
          notification["warning"]({
            message: "Not enough Storage Balance",
            description:
              "Your Storage Balance is not enough to sell on a new NFT. Please refill!",
          });
        }
      }
    } catch (e) {
      console.log("Transfer error: ", e);
      setTransferVisible(false);
    } finally {
      setTransferVisible(false);
    }
  }

  async function submitOnMint() {
    // call NFT contract mint_token
    try {
      await window.contractNFT.nft_mint(
        {
          //token_id: data.tokenId,
          receiver_id: window.accountId,
          // metadata: {
          //     title: data.tokenTitle,
          //     description: data.description,
          //     media: data.media
          // }
        },
        30000000000000,
        utils.format.parseNearAmount("0.01")
      );
    } catch (e) {
      console.log("Error: ", e);
    }
  }

  async function handleDeposit() {
    if (window.walletConnection.isSignedIn()) {
      await window.ftContract.near_deposit(
        {},
        200000000000000,
        utils.format.parseNearAmount("1")
      );
    } else {
      login();
    }
  }

  return (
    <div>
      <div className="ticket">
        <h1 className="ticket-tts">
          Your WNEAR Balance :{" "}
          {parseFloat(utils.format.formatNearAmount(balanceWrap)).toFixed(3)} wⓃ
        </h1>
      </div>
      <PageHeader
        className="site-page-header"
        title="My Stream Income"
        extra={[
          <Button onClick={handleDeposit} key="4">
            Register Wrap
          </Button>,
        ]}
      />

      <div
        style={{
          padding: 30,
          display: "flex",
          flexWrap: "wrap",
          alignContent: "stretch",
          maxWidth: "100%",
        }}
      >
        {console.log("dataStream2", dataStream)}
        {dataStream.map((item) => {
          return (
            <Card
              key={item.id}
              hoverable
              style={{
                width: 240,
                marginRight: 15,
                marginBottom: 15,
                flexWrap: "wrap",
              }}
              actions={[
                <b onClick={() => handleWithdraw(item.id)} key={"send"}>
                  <DollarCircleOutlined /> <br />
                  WithDraw
                </b>,
              ]}
            >
              <div style={{ fontSize: "20px" }}>
                {" "}
                Balance:{" "}
                {parseFloat(
                  utils.format.formatNearAmount(item.balance)
                ).toFixed(3)}{" "}
                wⓃ{" "}
              </div>
              <Card>
                {console.log("item", item)}
                Streamed:
                <Progress
                  percent={(
                    ((item.tokens_per_sec *
                      (new Date().getTime() -
                        item.timestamp_created / 1000000)) /
                      1000 /
                      item.balance) *
                    100
                  ).toFixed(1)}
                  strokeColor="CornflowerBlue"
                />{" "}
                <br />
                {(utils.format.formatNearAmount(item.tokens_per_sec) *
                  (new Date().getTime() - item.timestamp_created / 1000000)) /
                  1000 >
                parseFloat(utils.format.formatNearAmount(item.balance))
                  ? parseFloat(
                      utils.format.formatNearAmount(item.balance)
                    ).toFixed(3)
                  : (
                      (utils.format.formatNearAmount(item.tokens_per_sec) *
                        (new Date().getTime() -
                          item.timestamp_created / 1000000)) /
                      1000
                    ).toFixed(3)}
                wⓃ /
                {parseFloat(
                  utils.format.formatNearAmount(item.balance)
                ).toFixed(3)}
                wⓃ
              </Card>{" "}
              <br />
              <b>Total Withdraw: </b>
              {parseFloat(
                utils.format.formatNearAmount(item.tokens_total_withdrawn)
              ).toFixed(3)}{" "}
              wⓃ <br />
              <br />
              <b>OwnerID: </b>
              {item.owner_id} <br />
              <br />
              <b>ReceiverID: </b>
              {item.receiver_id} <br />
              <br />
              <b>Time Created: </b>
              <br />{" "}
              {new Date(item.timestamp_created / 1000000).toLocaleString()}{" "}
              <br />
              <br />
              <b>Tokens/Second:</b>
              <br />{" "}
              {parseFloat(
                utils.format.formatNearAmount(item.tokens_per_sec)
              ).toFixed(6)}{" "}
              wⓃ/sec <br />
              <br />
              <b>Status:</b> {item.status} <br />
              <br />
              <br />
            </Card>
          );
        })}
      </div>
      <ModalTransferNFT
        visible={transferVisible}
        handleOk={submitTransfer}
        handleCancel={() => setTransferVisible(false)}
      />
      <ModalSale
        visible={saleVisible}
        handleOk={submitOnSale}
        handleCancel={() => setSaleVisible(false)}
      />
      <ModalAuction
        visible={auctionVisible}
        handleOk={submitOnAuction}
        handleCancel={() => setAuctionVisible(false)}
      />
      <ModalMintNFT
        visible={mintVisible}
        handleOk={submitOnMint}
        handleCancel={() => setMintVisible(false)}
      />
      <ModalMatingNFT
        visible={mintVisible}
        handleOk={submitOnMint}
        handleCancel={() => setMatingVisible(false)}
      />
    </div>
  );
}

export default StreamIncome;
