# User Guide

The following is a user guide for how students can use the application.

## Signing In

The first step is to log in to the application.  
The login is handled by OAuth2 and is currently configured to use GitHub as the OAuth2 provider.  
Users are redirected to GitHub to sign in and then redirected back to the application.  
The OAuth2 provider can be changed in the future.  

![Gif of signing in with GitHub](./assets/sign_in.gif)

## Home Page

The home page contains a list of all the sprints for the current quarter.  
If a user successfully submitted a sprint, the sprint will be marked as completed.  
If the sprint was not submitted, the sprint will be marked as incomplete.  
All active sprints that can be submitted on the current date will be marked as available.

![Picture of the landing page showcasing the sprints](./assets/home_page.jpg)

If the OAuth2 authentication fails, the user will be redirected to the error page.

![Picture of the login error page](./assets/login_error.jpg)

If the frontend is unable to fetch the active sprints, an error messsge will be displayed.

![Picture of the fetch sprints error message](./assets/fetch_sprints_error.jpg)

## Team Report

On the home page, if a team report is open to submission, there will be blue arrow to the right that students can click on to complete a team report. As students answer the questions, the form will perform proactive validation. Once a student answered all the questions, they can submit the form. During the submission, the user will be redirected to the submission page, which will display the status of the submission.

![Gif of student submitting a team report](./assets/team_report.gif)

If the team report submission fails, an error message will be displayed.

![Picture of the submission error message](./assets/submit_team_report_error.jpg)

## Individual Report

Currently unavailable.  
Preview of what the page will look like.

![Picture of the individual report page](./assets/individual_report.jpg)

## Requirements Page

Currently unavailable.  
Preview of what the page will look like.

![Picture of the requirements page](./assets/requirements.jpg)