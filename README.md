# repository-wrapup
This is a project intended to help us wrap up our projects after they're no longer useful.

## Scoring

### Commit Age Score

A great way of telling whether or not a repository is still useful is to look at the age of the last commit. The older the commit, the less likely it is that the repository is still useful. This is because the repository is less likely to be abandoned if there have been commits recently.

There are three possible values for the score (list number is the score):

    0: This means that the repository has been updated in the last 30 days.
    1: The first step of the code is to retrieve the most recent commit date, and the date that the repository was created.
    2: The next step is to find the difference between the two dates, and then divide that by 2.
    3: The next step is to compare the most recent commit date to the date that the repository was created, plus the difference between the two dates, divided by 2.
    4: If the most recent commit date is before the date that the repository was created, plus the difference between the two dates, divided by 2, then the repository is empty.
    5: If the most recent commit date is after the date that the repository was created, plus the difference between the two dates, divided by 2, then the repository has been updated more recently than half the age of the repository, in days.
    6: If the most recent commit date is equal to the date that the repository was created, plus the difference between the two dates, divided by 2, then the most recent commit is older than half the age of the repository, in days.