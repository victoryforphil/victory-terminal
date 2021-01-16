import React from 'react';
import Head from 'next/Head';
import Link from 'next/Link';
import SerialComponent from "./serial"


const Home = () => {
  return (
    <React.Fragment>
      <SerialComponent>
        <h1>App is cool huh</h1>
      </SerialComponent>
    </React.Fragment>
  );
};

export default Home;
