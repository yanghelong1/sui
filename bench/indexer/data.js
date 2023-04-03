window.BENCHMARK_DATA = {
  "lastUpdate": 1680533977531,
  "repoUrl": "https://github.com/MystenLabs/sui",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "112846738+benr-ml@users.noreply.github.com",
            "name": "benr-ml",
            "username": "benr-ml"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1aadc807ec9f978379fb3cf2828b03450e2e816c",
          "message": "[improve readability] Rename VerifiedDigestCacheMetrics to SignatureVerifierMetrics (#10291)\n\n## Description \r\nRename VerifiedDigestCacheMetrics to SignatureVerifierMetrics\r\n\r\n## Test Plan \r\nAll tests pass",
          "timestamp": "2023-04-03T17:49:44+03:00",
          "tree_id": "b003188e755c8cda84d9e894c4a26b1e4c0e4241",
          "url": "https://github.com/MystenLabs/sui/commit/1aadc807ec9f978379fb3cf2828b03450e2e816c"
        },
        "date": 1680533975398,
        "tool": "cargo",
        "benches": [
          {
            "name": "persist_checkpoint",
            "value": 179065083,
            "range": "± 10223595",
            "unit": "ns/iter"
          },
          {
            "name": "get_checkpoint",
            "value": 313869,
            "range": "± 8215",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}