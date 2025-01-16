# 💥 Ignition - Robust Real-Time Event Streaming Service in Rust 🦀
  
[![Share on X](https://img.shields.io/badge/share-000000?logo=x&logoColor=white)](https://x.com/intent/tweet?text=Check%20out%20this%20project%20on%20GitHub:%20https://github.com/Axnjr/Ignite%20%23OpenIDConnect%20%23Security%20%23Authentication)
[![Share on Facebook](https://img.shields.io/badge/share-1877F2?logo=facebook&logoColor=white)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/Axnjr/Ignite)
[![Share on LinkedIn](https://img.shields.io/badge/share-0A66C2?logo=linkedin&logoColor=white)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/Axnjr/Ignite)
[![Share on Reddit](https://img.shields.io/badge/share-FF4500?logo=reddit&logoColor=white)](https://www.reddit.com/submit?title=Check%20out%20this%20project%20on%20GitHub:%20https://github.com/Axnjr/Ignite)
[![Share on Telegram](https://img.shields.io/badge/share-0088CC?logo=telegram&logoColor=white)](https://t.me/share/url?url=https://github.com/Axnjr/Ignite&text=Check%20out%20this%20project%20on%20GitHub)

# Overview 

This project was developed during my internship at **Dynamite**.  It serves as a **MonoRepo** for all the `Rust backend` components used in the project.  

### Other Components:
- [**The Web App**](https://github.com/Axnjr/Ignition-Web)  
- [**Client Pub-Sub SDK (JavaScript)**](https://github.com/Ignition-Dev/Js-Sdk)  
- [**Test Playground**](https://github.com/Ignition-Dev/Js-Sdk/tree/main/playground)  
- [**Lambda Functions**](https://github.com/Axnjr/dailyCronJob)  

---

# Example Usage 
Users need to get their `API_KEY` by creating their account. Hobby users get 100 daily requests and 10 con-current connections to exceed this limit users can subscribe to other paid plans. To interact with `Ignition` you can use the language specific SDK'S, untill now only JS SDK is available 😅, below is sample of how to use it:
```js
import Ignition from "ignition-js-sdk";

let ws = new Ignition({
    url: process.env.IGNITION_WSS_URL,
    apiKey: process.env.IGNITION_API_KEY,
    encryptionKey:"RADHA" // IF YOU WANT TO ENCRYPT YOUR MESSAGES. MESSAGES NEED TO BE DECRYPTED USING THE SAME KEY ON THE OTHER END !
})

ws.subscribe("test") // eventName

ws.on("test", (data) => { // eventName, callback
	console.log("message recived by `b`:",data)
})

ws.emit("test", "test", "hello world") // eventName: String, channelName: String, message: Any

ws.emit("test", "test", {
  "Name": "Axn",
  "Age": 21,
  "Occupation": "SDE"
})

ws.emit("tes", "test", 56);
```

# Data flow diagram 
![My First Board](https://github.com/user-attachments/assets/d3d8df0f-5b2e-4577-b48d-e4cc49b9e6f3)

---

# Architecture 

- **Hobby** and **Pro** requests are handled by the [`ignition_shared_v5`](https://github.com/Axnjr/Ignite/tree/main/ignition_shared_v5) container, which is deployed using **AWS Elastic Container Service (ECS)**.  
- For **Enterprise** clients, a dedicated instance is provisioned with the [`dedicated_v2`](https://github.com/Axnjr/Ignite/tree/main/WssDedicated) container for optimal performance. 💥

---

# Subscription Model 

<table border="0" cellspacing="14" cellpadding="24" style="width: 100%; text-align: center;">
  <thead style="background-color: #f2f2f2;">
    <tr>
      <th>Plan</th>
      <th>Daily Requests</th>
      <th>Concurrent Connections</th>
      <th>Additional Features</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>Hobby (Free)</strong></td>
      <td>Up to 100</td>
      <td>10</td>
      <td>—</td>
    </tr>
    <tr>
      <td><strong>Pro (Paid)</strong></td>
      <td>Up to 10,000</td>
      <td>500</td>
      <td>—</td>
    </tr>
    <tr>
      <td><strong>Enterprise</strong></td>
      <td>Unlimited</td>
      <td>Unlimited</td>
      <td>
        <ul>
          <li>No rate-limiting or authentication overhead.</li>
          <li>Enhanced data security and governance.</li>
          <li>Minimal latency (25ms).</li>
          <li>Custom logging and configuration.</li>
        </ul>
      </td>
    </tr>
  </tbody>
</table>

---

# Features

- Scalable real-time event streaming built with **Rust**.
- Data security and privacy using `AES Encryption`.
- Designed for high performance and low latency.  
- Flexible subscription plans for various use cases.  
- Easily deployable and maintainable with AWS ECS.
- Most affordable real-time service compared from: [Ably](https://ably.com/), [Pusher](https://pusher.com/), etc

---

# Upcomming features
- `XMPP Protocol` (What whatsapp uses) based event transmision
- `WebRTC` implementation for audio, video data.
- `SDK's` in other languages like Python, Java, Rust, Golang, Php, Ruby, etc ..

# Docker Hub links 

#### [**Public Shared Server**](https://hub.docker.com/r/axnjr/ignition_shared)
```
docker pull axnjr/ignition_shared:v5
```

#### [**Dedicated Private Server**](https://hub.docker.com/r/axnjr/ignition_wssd)
```
docker pull axnjr/ignition_wssd
```

# Feel free to explore and contribute!   
