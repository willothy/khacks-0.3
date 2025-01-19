import { WebRTCAdaptor } from "@antmedia/webrtc_adaptor";
import { useCallback, useEffect, useRef, useState } from "react";

export default function Playback({
  robotIpAddr,
  className,
}: {
  robotIpAddr: string;
  className?: string;
}) {
  const [websocketConnected, setWebsocketConnected] = useState(false);
  const webRTCAdaptor = useRef<WebRTCAdaptor | null>(null);
  const playingStream = useRef<string | null>(null);

  const handleStopPlaying = useCallback(() => {
    if (playingStream.current !== null) {
      webRTCAdaptor.current?.stop(playingStream.current);
    }
  }, []);

  useEffect(() => {
    if (webRTCAdaptor.current === undefined || webRTCAdaptor.current === null) {
      webRTCAdaptor.current = new WebRTCAdaptor({
        websocket_url: `http://${robotIpAddr}:8083/stream/s1/channel/0/webrtc?uuid=s1&channel=0`,
        mediaConstraints: {
          video: true,
          audio: true,
        },
        peerconnection_config: {
          // iceServers: [{ urls: "stun:stun1.l.google.com:19302" }],
        },
        sdp_constraints: {
          OfferToReceiveAudio: true,
          OfferToReceiveVideo: true, // Set to true to receive video
        },
        remoteVideoId: "remoteVideo",
        callback: (info: string) => {
          if (info === "initialized") {
            setWebsocketConnected(true);
            webRTCAdaptor.current?.play("0");
          }
        },
        callbackError: (error: string, message: string) => {
          console.log(error, message);
          if (error === "no_stream_exist") {
            handleStopPlaying();
            alert(error);
          }
        },
      });
    }
  }, [handleStopPlaying, robotIpAddr]);

  return (
    <div className={`flex bg-white ${className}`}>
      <span className="absolute">
        {websocketConnected ? null : "Not connected"}
      </span>
      <video id="remoteVideo" muted autoPlay playsInline />
    </div>
  );
}
