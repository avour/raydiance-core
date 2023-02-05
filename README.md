<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->




<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Radiance Protocol</h3>

  <p align="center">
    <br />
    <a href="https://radaince-protocol.web.app/#/pools"><strong>View Live Website »</strong></a>
    <br />
    <br />
    <a href="https://youtu.be/-IDu9lG-wZM">View Video Demo</a>
    ·
    <a href="https://github.com/avour/radiance-frontend">View Frontend Code</a>
    ·
  </p>
</div>




<!-- ABOUT THE PROJECT -->
## About The Project

Radiance Protocol is a permissionless lending market that offers a unique solution to the problems of impermanent loss and lost collateral opportunity in the DeFi space. Radiance supports markets available on OpenBook https://www.openbook-solana.com/

## Why Radiance 

Impermanent loss is a major concern in the DeFi lending market, deterring many risk-averse investors from entering the market. Radiance solves this problem by offering a platform where LP tokens can be used as collateral to borrow other tokens, reducing the risk of impermanent loss and earning yield in the process. Borrowers can also leverage their LP tokens to lock up assets and increase their yield.

## Features 

- Use LP tokens as collateral to borrow other tokens 

[![Product Name Screen Shot][screenshot1]](https://example.com)


- Earn yield by lending tokens to borrowers 

[![Product Name Screen Shot][screenshot3]](https://example.com)


- Reduce risk of impermanent loss 

[![Product Name Screen Shot][screenshot4]](https://example.com)


- Borrowers can leverage their LP tokens to lock up assets and increase their yield 

[![Product Name Screen Shot][screenshot5]](https://example.com)

## Vision 

Radiance aims to become the world's top lending marketplace based on LP tokens, solving the problems of lost collateral opportunity and impermanent loss in the current LP market. Join us on our mission to revolutionize the DeFi lending market.



<p align="right">(<a href="#readme-top">back to top</a>)</p>

## API Reference

### Create a pool
`create_pool(ctx: Context<CreatePool>, input: CreatePoolInput) -> Result<()>`

This function allows the caller to create a new pool in the Radiance lending market.

### Deposit collateral
`deposit_collateral(ctx: Context<DepositCollateral>, input: DepositCollateralInput) -> Result<()>`

This function allows the caller to deposit collateral to an existing pool in the Radiance lending market.

### Withdraw collateral
`withdraw_collateral(ctx: Context<WithdrawCollateral>, input: WithdrawCollateralInput) -> Result<()>`

This function allows the caller to withdraw collateral from an existing pool in the Radiance lending market.

### Borrow tokens
`borrow(ctx: Context<Borrow>, input: BorrowInput) -> Result<()>`

This function allows the caller to borrow tokens from an existing pool in the Radiance lending market.

### Repay loan
`repay_loan(ctx: Context<RepayLoan>, input: RepayLoanInput) -> Result<()>`

This function allows the caller to repay a loan that was previously taken from an existing pool in the Radiance lending market.

### Supply borrowable tokens
`supply_borrowable(ctx: Context<SupplyBorrowable>, input: SupplyBorrowableInput) -> Result<()>`

This function allows the caller to add to the supply of borrowable tokens in an existing pool in the Radiance lending market.

### Withdraw borrowable tokens
`withdraw_borrowable(ctx: Context<WithdrawBorrowable>, input: WithdrawBorrowableInput) -> Result<()>`

This function allows the caller to withdraw borrowable tokens from an existing pool in the Radiance lending market.


<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/othneildrew/Best-README-Template.svg?style=for-the-badge
[contributors-url]: https://github.com/othneildrew/Best-README-Template/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/othneildrew/Best-README-Template.svg?style=for-the-badge
[forks-url]: https://github.com/othneildrew/Best-README-Template/network/members
[stars-shield]: https://img.shields.io/github/stars/othneildrew/Best-README-Template.svg?style=for-the-badge
[stars-url]: https://github.com/othneildrew/Best-README-Template/stargazers
[issues-shield]: https://img.shields.io/github/issues/othneildrew/Best-README-Template.svg?style=for-the-badge
[issues-url]: https://github.com/othneildrew/Best-README-Template/issues
[license-shield]: https://img.shields.io/github/license/othneildrew/Best-README-Template.svg?style=for-the-badge
[license-url]: https://github.com/othneildrew/Best-README-Template/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/othneildrew
[product-screenshot]: images/screenshot.png
[screenshot1]: images/screenshot1.png
[screenshot2]: images/screenshot2.png
[screenshot3]: images/screenshot3.png
[screenshot4]: images/screenshot4.png
[screenshot5]: images/screenshot5.png
[screenshot6]: images/screenshot6.png
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com 
