# RSX

## A rusty implementation of [JSX](https://reactjs.org/docs/introducing-jsx.html)

### [Examples](./examples/)

### Format

```jsx
<html>
    <head>
        <title>"Hello World"</title>
    </head>
    <body>
        <h1>"Hello World"</h1>
        {
            (0..4).map(|_|
                rsx!(
                    <p class="emphasis">
                        ">o_o<"
                    </p>
                )
            )
        }
    </body>
</html>
```

### HTTPS/TLS

```sh
# How to generate a self-signed certificate and a private key
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
```
