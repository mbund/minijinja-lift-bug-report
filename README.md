This is the output of `cargo run`:

```
        global exists

         <wrapper>
            v_a
            global DOES NOT exist
        </wrapper>

         <wrapper>

            v_a
            global exists
        </wrapper>
```

The bug is that in the first `<wrapper>` block, the

