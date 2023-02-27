# DNS-in-Rust

Build up a simple DNS with Rust

## How to use

Start your local DNS.
> Port is 2054 in default.

```console
$ cargo run
...
Received query: DnsQuestion { name: "google.com", qtype: A }
attempting lookup of A google.com with ns 198.41.0.4
attempting lookup of A google.com with ns 192.12.94.30
attempting lookup of A google.com with ns 216.239.34.10
Answer: A { domain: "google.com", addr: 142.251.42.238, ttl: 300 }
```

Query a specific host name from the local DNS.

```console
$ dig @127.0.0.1 -p 2054 google.com

; <<>> DiG 9.10.6 <<>> @127.0.0.1 -p 2054 google.com
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 10724
;; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0

;; QUESTION SECTION:
;google.com.                    IN      A

;; ANSWER SECTION:
google.com.             300     IN      A       142.251.42.238

;; Query time: 100 msec
;; SERVER: 127.0.0.1#2054(127.0.0.1)
;; WHEN: Mon Feb 27 10:31:45 CST 2023
;; MSG SIZE  rcvd: 54
```

## TODOs

- [ ] Cache

## References

- [dnsguide](https://github.com/EmilHernvall/dnsguide)
- [RFC1034](https://www.rfc-editor.org/rfc/rfc1034)
- [RFC1035](https://www.rfc-editor.org/rfc/rfc1035)
- [DNS Recursive vs Iterative](https://www.cloudflare.com/zh-tw/learning/dns/what-is-recursive-dns/)
