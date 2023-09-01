use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-env-changed=LND_REPO_DIR");
    let dir = match std::env::var_os("LND_REPO_DIR") {
        Some(lnd_repo_path) => {
            let mut lnd_rpc_dir = PathBuf::from(lnd_repo_path);
            lnd_rpc_dir.push("lnrpc");
            lnd_rpc_dir
        }
        None => PathBuf::from("vendor"),
    };

    let lnd_rpc_proto_file = dir.join("lightning.proto");
    println!("cargo:rerun-if-changed={}", lnd_rpc_proto_file.display());

    let protos = [
        "signrpc/signer.proto",
        "walletrpc/walletkit.proto",
        "lightning.proto",
    ];

    let proto_paths: Vec<_> = protos
        .iter()
        .map(|proto| {
            let proto_path = dir.join(proto);
            proto_path
        })
        .collect();

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(false)
        .type_attribute(".lnrpc.Feature", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.Chain", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.GetInfoResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.Channel", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.ConnectPeerResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.ChannelPoint", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.ChannelConstraints", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.HTLC", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.NewAddressResponse", "#[derive(serde::Serialize)]")
        .type_attribute(
            ".lnrpc.WalletBalanceResponse",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(".lnrpc.WalletAccountBalance", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.ListPeersResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.Peer", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.TimestampedError", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.AddInvoiceResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.SendResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.Route", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.Hop", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.MPPRecord", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.AMPRecord", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.ListInvoiceResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.Invoice", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.RouteHint", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.HopHint", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.InvoiceHTLC", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.InvoiceHTLCState", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.AMP", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.AMPInvoiceState", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.ListPaymentsResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.Payment", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.HTLCAttempt", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.Failure", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.PaymentFailureReason", "#[derive(serde::Serialize)]")
        .type_attribute(".lnrpc.ChannelUpdate", "#[derive(serde::Serialize)]")
        .type_attribute(
            ".lnrpc.PendingChannelsResponse",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(".lnrpc.PendingHTLC", "#[derive(serde::Serialize)]")
        .compile(&proto_paths, &[dir])?;
    Ok(())
}
