import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

const FeatureList = [
    {
        title: 'Easy to Use',
        Svg: require('@site/static/img/hourglass-svgrepo-com.svg').default,
        description: (
            <>
                Svelt is designed to simplify your workflow. With intuitive component generation
                and ready-to-use features, you can focus on building without worrying about complex configurations.
            </>
        ),
    },
    {
        title: 'Powerful Initialization',
        Svg: require('@site/static/img/dna-svgrepo-com.svg').default,
        description: (
            <>
                Choose from a variety of configurations during initialization: TypeScript, Tailwind CSS, and more.
                Svelt saves you valuable time by giving you a solid foundation right from the start.
            </>
        ),
    },
    {
        title: 'Blazing Performance',
        Svg: require('@site/static/img/rocket-svgrepo-com.svg').default,
        description: (
            <>
                Built with Rust, Svelt delivers exceptional speed and reliability. Enjoy a smooth developer experience,
                even for the most complex projects, without compromising on performance.
            </>
        ),
    },
];

function Feature({Svg, title, description}) {
    return (
        <div className={clsx('col col--4')}>
            <div className="text--center">
                <Svg className={styles.featureSvg} role="img"/>
            </div>
            <div className="text--center padding-horiz--md">
                <Heading as="h3">{title}</Heading>
                <p>{description}</p>
            </div>
        </div>
    );
}

export default function HomepageFeatures() {
    return (
        <section className={styles.features}>
            <div className="container">
                <div className="row">
                    {FeatureList.map((props, idx) => (
                        <Feature key={idx} {...props} />
                    ))}
                </div>
            </div>
        </section>
    );
}
