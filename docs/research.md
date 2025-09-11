# research

## model_notes: gpt-oss

- https://arxiv.org/pdf/2508.10925
- https://openai.com/index/gpt-oss-model-card/
- https://cookbook.openai.com/articles/openai-harmony#prompt-format
- https://github.com/openai/harmony

Opted for gpt-oss-20b due to reduced system specification requirements to run locally, availability of Rust harmony client interfacing, and open weights design.

###  model_layout

```
gpt-oss-20b-unsloth-bnb-4bit on  main
❯ tree
.
├── chat_template.jinja
├── chat_template.json
├── config.json
├── generation_config.json
├── model-00001-of-00004.safetensors
├── model-00002-of-00004.safetensors
├── model-00003-of-00004.safetensors
├── model-00004-of-00004.safetensors
├── model.safetensors.index.json
├── README.md
├── special_tokens_map.json
├── tokenizer_config.json
└── tokenizer.json
```
