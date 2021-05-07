# Manoonchai

Modern &amp; Productive Thai keyboard layout

[Try me on Manoontype!](https://manoontype.web.app) or [See the preview of the layout & optimization score](https://carpalx-th.vercel.app)

## v0.3 (May 6, 2021)

[Download printable format](https://github.com/narze/Manoonchai/files/6438811/layout.pdf)

```plaintext
["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "="]
["ใ", "ต", "ห", "ล", "ส", "ป", "ั", "ก", "ิ", "บ", "็", "ฬ", "ฯ"]
["ง", "เ", "ร", "น", "ม", "อ", "า", "่", "้", "ว", "ื"]
["ุ", "ไ", "ท", "ย", "จ", "ค", "ี", "ด", "ะ", "ู"]
["!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+"]
["ฒ", "ฏ", "ซ", "ญ", "ฟ", "ฉ", "ึ", "ธ", "ฐ", "ฎ", "ฆ", "ฑ", "ฌ"]
["ษ", "ถ", "แ", "ช", "พ", "ผ", "ำ", "ข", "โ", "ภ", "\""]
["ฤ", "ฝ", "ๆ", "ณ", "๊", "๋", "์", "ศ", "ฮ", "?"]
```

- Contributed by ([@iNViTiON](https://github.com/iNViTiON))
- Replaced `฿` with `"`
- Moved `฿`, `"`, `ฑ`, `ฌ`, `ฬ`, `ฯ` to the right most
- Effort is slightly reduced, but acceptable for real world usage

![image](https://user-images.githubusercontent.com/248741/117223218-66704480-ae37-11eb-9ee5-0ae8556f4d02.png)

<details>
  <summary>Previous Versions</summary>

## v0.2 (May 1, 2021)

```plaintext
["1","2","3","4","5","6","7","8","9","0","-","="]
["พ","ค","ย","ว","ล","ป","ั","ก","ต","บ","็","ู","์"]
["ห","เ","น","ร","ม","อ","า","่","้","ง","ื"]
["ช","ไ","ส","ท","จ","ิ","ี","ด","ะ","ุ"]
["!","@","#","$","%","^","&","*","(",")","_","+"]
["ฑ","ฒ","ษ","ญ","ฟ","ฎ","ฉ","ภ","ฐ","ฤ","ฆ","ฌ","ฯ"]
["ๆ","ถ","แ","ข","ผ","ึ","ใ","ำ","โ","ศ","ฮ"]
["ฬ","๋","๊","ซ","ฝ","?","ณ","ธ","ฏ","฿"]
```

After model optimization, weight adjustment and more than 10 million iterations, this layout achieve 49.5% improvement from Kedmanee.

![image](https://user-images.githubusercontent.com/248741/116779603-856e7f80-aaa1-11eb-9175-26d0802d0bd9.png)

## v0.1 (April 24, 2021)

Generated from [Carpalx-th](https://github.com/narze/carpalx-th), and edited by hand to make it more sensible. AltGr layer not available yet so `ฃ`, `ฅ` will be missing.

```plaintext
["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "="]
["ู", "พ", "ง", "ส", "ต", "ค", "ั", "อ", "บ", "ป", "็", "ๆ", "ฐ"]
["ว", "ก", "น", "ร", "ย", "เ", "่", "า", "ม", "ี", "ะ"]
["ท", "ใ", "ห", "ล", "ช", "ไ", "้", "ด", "ุ", "์"]
["!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+"]
["ฯ", "ฏ", "ษ", "ศ", "ซ", "๊", "โ", "ฬ", "ภ", "ฮ", "ฒ", "ฤ", "ฑ"]
["ธ", "ข", "แ", "ญ", "จ", "ถ", "ิ", "ื", "ำ", "ึ", "๋"]
["ฆ", "ฌ", "ฉ", "ผ", "ฝ", "฿", "ณ", "ฟ", "ฎ", "?"]
```

![image](https://user-images.githubusercontent.com/248741/115959989-383a5d00-a539-11eb-86e5-0a70b23a999e.png)
</details>

## Why?

Most of the problems are like problems with QWERTY layout, both Kedmanee & Pattachote layouts are:

- Invented for typewriters which have been long gone these days
- Very old (50+ years!)
- tba

## Key Features

- Use Arabic numbers by default
- No Thai alphabets on number row
- Tested with modern dataset
- Support all keyboard form factors down to 40%
