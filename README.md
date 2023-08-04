# repository-wrapup

This is a project intended to help us wrap up our projects after they're no longer useful.

## The Problem

Over the lifespan of an organization, it is easy to leave a lot of old code lying around. Sometimes these are code repositories that we ran in production that are no longer used, sometimes they're one-off tools, and sometimes they're forks that we haven't really done anything with in a while. Regardless of how the repository began, it is no longer meeting the needs it once did (or hoped to).

On top of these repositories being "cruft," they also can contain sensitive information. Commit history may contain a secret, or might have PII in it regarding employees or former employees. It's not always clear what the concrete risks are, but there _is_ risk, and removing the repositories can reduce or even fully mitigate that risk.

This program's goal is to aid you in identifying and removing these repositories.

## The Goals

1. Help you identify repositories in an organization which haven't been touched for a while, or no longer have any committers in your organization.
2. Help you identify forks which you haven't updated in a while, and may need to be deleted or rebased from their origin.
3. Eventually, help you automate the process of archiving or removing reposiories in your organization.

<!--
Commenting this out for now
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
-->