# RUST API TODO

## Utils

### JWT

#### Implementar

- OAuth for _google, discord, twitter_
  - criar rota Auth/OAuth
  - para permitir que usarios entrem com contas como **Discord, Google, entre outros**

---

#### Admin

> [!CAUTION]  
> Refactor return type and inline responses
> Refactor code implementation in handlers from other files

- [ ] - Review admin functions return
- [ ] - Review inline HttpResponses

---

#### Verify Ownership

> [!NOTE]  
> Improve function logic

- [ ] - Review function return
- [ ] - Review inline HttpResponses

---

#### Verify JWT

> [!NOTE]  
> Improve code implementation in return and inline responses

- [ ] - Review jwt verification function return
- [ ] - Review inline HttpResponses

## API Features

### News

> [!WARNING]  
> Finish comments, likes and views feature

- [x] - Article Model
- [x] - Articles Handler
- [x] - Articles SQL
- [x] - Articles Table

---

- [ ] - Comment Model
- [ ] - Comments Handler
- [ ] - Comments SQL
- [ ] - Comments Table

---

- [ ] - Like Model
- [ ] - Likes Handler
- [ ] - Likes SQL
- [ ] - Likes Table

---

- [ ] - View Model
- [ ] - Views Handler
- [ ] - Views SQL
- [ ] - Views Table

---

- [ ] - Tag Model
- [ ] - Tags Handler
- [ ] - Tags SQL
- [ ] - Tags Table

---

### Blog

> [!NOTE]  
> Refactor models and handlers

- [ ] - Blog Model
- [ ] - Blogs Handler
- [ ] - Blogs SQL
- [ ] - Blogs Table

---

- [ ] - Comment Model
- [ ] - Comments Handler
- [ ] - Comments SQL
- [ ] - Comments Table

---

- [ ] - Like Model
- [ ] - Likes Handler
- [ ] - Likes SQL
- [ ] - Likes Table

---

- [ ] - View Model
- [ ] - Views Handler
- [ ] - Views SQL
- [ ] - Views Table

---

- [ ] - Tag Model
- [ ] - Tags Handler
- [ ] - Tags SQL
- [ ] - Tags Table

---

### Youtube / Cos.tv _video stream service_

> [!TIP]  
> Implement from scratch

- [ ] - Youtube Model
- [ ] - Youtubes Handler
- [ ] - Youtubes SQL
- [ ] - Youtubes Table

---

- [ ] - Comment Model
- [ ] - Comments Handler
- [ ] - Comments SQL
- [ ] - Comments Table

---

- [ ] - Like Model
- [ ] - Likes Handler
- [ ] - Likes SQL
- [ ] - Likes Table

---

- [ ] - View Model
- [ ] - Views Handler
- [ ] - Views SQL
- [ ] - Views Table

---

- [ ] - Playlist Model
- [ ] - Playlists Handler
- [ ] - Playlists SQL
- [ ] - Playlists Table

### Password Manager

> [!TIP]  
> Implement from scratch

- [ ] - Account Model
- [ ] - Accounts Handler
- [x] - Password Encryption Algorithm
- [ ] - Accounts SQL
- [ ] - Accounts Table

### Twitch.tv / Kicks _live stream service_

> [!TIP]  
> Implement from scratch

- [ ] - Youtube Model
- [ ] - Youtubes Handler
- [ ] - Youtubes SQL
- [ ] - Youtubes Table

---

- [ ] - Comment Model
- [ ] - Comments Handler
- [ ] - Comments SQL
- [ ] - Comments Table

---

- [ ] - Like Model
- [ ] - Likes Handler
- [ ] - Likes SQL
- [ ] - Likes Table

---

- [ ] - View Model
- [ ] - Views Handler
- [ ] - Views SQL
- [ ] - Views Table

---

### Store _Ecommerce service_

> [!CAUTION]  
> Refactor from scratch

- [ ] - Store Model
- [ ] - Stores Handler
- [ ] - Stores SQL
- [ ] - Stores Table

---

- [ ] - Product Model
- [ ] - Products Handler
- [ ] - Products SQL
- [ ] - Products Table

---

- [ ] - Order Model
- [ ] - Orders Handler
- [ ] - Orders SQL
- [ ] - Orders Table

---

- [ ] - Review Model
- [ ] - Reviews Handler
- [ ] - Reviews SQL
- [ ] - Reviews Table

---

## Refactor

- [x] - Decrease "callback hell" occurrences
