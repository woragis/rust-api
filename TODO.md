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

#### News Articles

- [x] - Article Model
- [x] - Articles Handler
- [x] - Articles SQL
- [x] - Articles Table

#### News Comments

- [ ] - Comment Model
- [x] - Comments Handler
- [ ] - Comments SQL
- [x] - Comments Table

#### News Likes

- [ ] - Like Model
- [x] - Likes Handler
- [ ] - Likes SQL
- [x] - Likes Table

#### News Views

- [ ] - View Model
- [ ] - Views Handler
- [ ] - Views SQL
- [x] - Views Table

#### News Tags

- [ ] - Tag Model
- [ ] - Tags Handler
- [ ] - Tags SQL
- [x] - Tags Table

---

### Blog

> [!NOTE]  
> Refactor models and handlers

#### Blogs

- [ ] - Blog Model
- [ ] - Blogs Handler
- [ ] - Blogs SQL
- [ ] - Blogs Table

#### Blog Comments

- [ ] - Comment Model
- [ ] - Comments Handler
- [ ] - Comments SQL
- [ ] - Comments Table

#### Blog Likes

- [ ] - Like Model
- [ ] - Likes Handler
- [ ] - Likes SQL
- [ ] - Likes Table

#### Blog Views

- [ ] - View Model
- [ ] - Views Handler
- [ ] - Views SQL
- [ ] - Views Table

#### Blog Tags

- [ ] - Tag Model
- [ ] - Tags Handler
- [ ] - Tags SQL
- [ ] - Tags Table

---

### Youtube / Cos.tv _video stream service_

> [!TIP]  
> Implement from scratch

#### Youtube Video Stream

- [ ] - Youtube Model
- [ ] - Youtubes Handler
- [ ] - Youtubes SQL
- [ ] - Youtubes Table

#### Youtube Comments

- [ ] - Comment Model
- [ ] - Comments Handler
- [ ] - Comments SQL
- [ ] - Comments Table

#### Youtube Likes

- [ ] - Like Model
- [ ] - Likes Handler
- [ ] - Likes SQL
- [ ] - Likes Table

#### Youtube Views

- [ ] - View Model
- [ ] - Views Handler
- [ ] - Views SQL
- [ ] - Views Table

#### Youtube Playlists

- [ ] - Playlist Model
- [ ] - Playlists Handler
- [ ] - Playlists SQL
- [ ] - Playlists Table

---

### Password Manager

> [!TIP]  
> Implement from scratch

- [ ] - Account Model
- [x] - Accounts Handler
- [x] - Password Encryption Algorithm
- [ ] - Accounts SQL
- [ ] - Accounts Table

### Twitch.tv / Kicks _live stream service_

> [!TIP]  
> Implement from scratch

#### Lives

- [ ] - Live Model
- [ ] - Lives Handler
- [ ] - Lives SQL
- [ ] - Lives Table

#### Lives Comments

- [ ] - Comment Model
- [ ] - Comments Handler
- [ ] - Comments SQL
- [ ] - Comments Table

#### Lives Likes

- [ ] - Like Model
- [ ] - Likes Handler
- [ ] - Likes SQL
- [ ] - Likes Table

#### Lives Views

- [ ] - View Model
- [ ] - Views Handler
- [ ] - Views SQL
- [ ] - Views Table

---

### Store _Ecommerce service_

> [!CAUTION]  
> Refactor from scratch

#### Stores

- [ ] - Store Model
- [ ] - Stores Handler
- [ ] - Stores SQL
- [ ] - Stores Table

#### Store Products

- [ ] - Product Model
- [ ] - Products Handler
- [ ] - Products SQL
- [ ] - Products Table

#### Store Orders

- [ ] - Order Model
- [ ] - Orders Handler
- [ ] - Orders SQL
- [ ] - Orders Table

#### Store Riviews

- [ ] - Review Model
- [ ] - Reviews Handler
- [ ] - Reviews SQL
- [ ] - Reviews Table

---

## Refactor

- [x] - Decrease "callback hell" occurrences
