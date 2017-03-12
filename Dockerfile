FROM ruby:2.6

# throw errors if Gemfile has been modified since Gemfile.lock
RUN bundle config --global frozen 1

WORKDIR /usr/src/app

COPY Gemfile Gemfile.lock ./
RUN bundle install

COPY . .

RUN chmod +x ./mirrors
RUN apt-get update && apt-get -y install git

ENV PATH="/usr/src/app:${PATH}"

WORKDIR /work

CMD ["mirrors"]